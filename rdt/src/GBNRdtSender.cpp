#include "GBNRdtSender.h"
#include "DataStructure.h"
#include "Global.h"
#include "RandomEventEnum.h"
#include <cstring>

GBNRdtSender::GBNRdtSender()
    : nextseqnum(0), base(0), window(), window_len(WINDOW_LENGTH),
      seq_len(2 * WINDOW_LENGTH) {}

GBNRdtSender::~GBNRdtSender() {}

bool GBNRdtSender::getWaitingState() { return window.size() == window_len; }

bool GBNRdtSender::send(const Message &message) {
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
    pUtils->printPacket("[GBN Sender]: sending a packet.", packet);

    if (base == packet.seqnum) {
      pns->startTimer(SENDER, Configuration::TIME_OUT, base);
    }
    pns->sendToNetworkLayer(RECEIVER, packet);
    nextseqnum = (nextseqnum + 1) % seq_len;
    return true;
  }
}

void GBNRdtSender::receive(const Packet &ack_packet) {
  int checkSum = pUtils->calculateCheckSum(ack_packet);

  if (checkSum == ack_packet.checksum) {
    pns->stopTimer(SENDER, base);
    pUtils->printPacket("[GBN Sender]: ACK packet received successfully.",
                        ack_packet);
    fprintf(stdout, "[GBN Sender]: Window status:\n[ ");
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
    if (!window.empty()) {
      pns->startTimer(SENDER, Configuration::TIME_OUT, base);
    }
  } else {
    pUtils->printPacket("[GBN Sender]: ACK packet received corrupted.",
                        ack_packet);
  }
}

void GBNRdtSender::timeoutHandler(int seqNum) {
  pns->stopTimer(SENDER, seqNum);
  pns->startTimer(SENDER, Configuration::TIME_OUT, seqNum);
  for (auto p : window) {
    pns->sendToNetworkLayer(RECEIVER, p);
    pUtils->printPacket("[GBN Sender]: timeouts, resending the packet.", p);
  }
}
