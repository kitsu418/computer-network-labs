#include "TCPRdtSender.h"
#include "DataStructure.h"
#include "Global.h"
#include "RandomEventEnum.h"
#include <cstdio>
#include <cstring>

TCPRdtSender::TCPRdtSender()
    : nextseqnum(0), base(0), window(), window_len(WINDOW_LENGTH), dup_num(0),
      seq_len(2 * WINDOW_LENGTH) {}

TCPRdtSender::~TCPRdtSender() {}

bool TCPRdtSender::getWaitingState() { return window.size() == window_len; }

bool TCPRdtSender::send(const Message &message) {
  if (getWaitingState()) {
    return false;
  } else {
    Packet packet = {};
    packet.acknum = -1;
    packet.checksum = 0;
    packet.seqnum = nextseqnum;
    memcpy(packet.payload, message.data, sizeof(message.data));
    packet.checksum = pUtils->calculateCheckSum(packet);
    window.emplace_back(packet);
    pUtils->printPacket("[TCP Sender]: sending a packet.", packet);

    if (base == packet.seqnum) {
      pns->startTimer(SENDER, Configuration::TIME_OUT, base);
    }
    pns->sendToNetworkLayer(RECEIVER, packet);
    nextseqnum = (nextseqnum + 1) % seq_len;
    return true;
  }
}

void TCPRdtSender::receive(const Packet &ack_packet) {
  int checkSum = pUtils->calculateCheckSum(ack_packet);
  int window_idx = (ack_packet.acknum - base + seq_len) % seq_len;
  printf("Window_idx: %d\n", window_idx);

  if (checkSum == ack_packet.checksum && window_idx < window.size()) {
    pns->stopTimer(SENDER, base);
    pUtils->printPacket("[TCP Sender]: ACK packet received successfully.",
                        ack_packet);
    fprintf(stdout, "[TCP Sender]: Window status:\n[ ");
    for (int i = 0; i < window.size(); i++) {
      fprintf(stdout, "%d ", (base + i) % this->seq_len);
    }
    while (base != (ack_packet.acknum + 1) % seq_len) {
      base = (base + 1) % seq_len;
      window.pop_front();
    }
    fprintf(stdout, "] =>\n[ ");
    for (int i = 0; i < window.size(); i++) {
      fprintf(stdout, "%d ", (base + i) % this->seq_len);
    }
    fprintf(stdout, "]\n");
    dup_num = 0;
    if (!window.empty()) {
      pns->startTimer(SENDER, Configuration::TIME_OUT, base);
    }
  } else if (checkSum != ack_packet.checksum) {
    pUtils->printPacket("[TCP Sender]: ACK packet received corrupted.",
                        ack_packet);
  } else if (ack_packet.acknum == (base - 1 + seq_len) % seq_len) {
    pUtils->printPacket("[TCP Sender]: ACK packet has been received before.",
                        ack_packet);
    ++dup_num;
    printf("dup_dum: %d\n", dup_num);
    if (dup_num == 3 && !window.empty()) {
      pUtils->printPacket("[TCP Sender]: Quick resend.", window.front());
      pns->sendToNetworkLayer(RECEIVER, window.front());
      dup_num = 0;
    }
  } else {
    printf("ack_num: %d\n", ack_packet.acknum);
  }
}

void TCPRdtSender::timeoutHandler(int seqNum) {
  pUtils->printPacket("[TCP Sender]: Timeout, resending packet.",
                      window.front());
  pns->stopTimer(SENDER, seqNum);
  pns->startTimer(SENDER, Configuration::TIME_OUT, seqNum);
  pns->sendToNetworkLayer(RECEIVER, window.front());
}
