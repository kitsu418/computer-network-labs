#include "SRRdtSender.h"
#include "DataStructure.h"
#include "Global.h"
#include "RandomEventEnum.h"
#include <cstring>

SRRdtSender::SRRdtSender()
    : nextseqnum(0), window(), sendbase(0), window_len(WINDOW_LENGTH),
      seq_len(2 * WINDOW_LENGTH) {}

SRRdtSender::~SRRdtSender() {}

bool SRRdtSender::getWaitingState() { return window.size() == window_len; }

bool SRRdtSender::send(const Message &message) {
  if (getWaitingState()) {
    return false;
  } else {
    Packet packet = {};
    packet.acknum = -1;
    packet.checksum = 0;
    packet.seqnum = nextseqnum;
    memcpy(packet.payload, message.data, sizeof(message.data));
    packet.checksum = pUtils->calculateCheckSum(packet);
    window.push_back(std::make_pair(packet, false));
    pUtils->printPacket("[SR Sender]: sending a packet.", packet);

    pns->startTimer(SENDER, Configuration::TIME_OUT, nextseqnum);
    pns->sendToNetworkLayer(RECEIVER, packet);
    nextseqnum = (nextseqnum + 1) % seq_len;
    return true;
  }
}

void SRRdtSender::receive(const Packet &ack_packet) {
  int checkSum = pUtils->calculateCheckSum(ack_packet);
  int window_idx = (ack_packet.acknum - sendbase + seq_len) % seq_len;

  if (checkSum == ack_packet.checksum && window_idx < window.size() &&
      window[window_idx].second == false) {
    window[window_idx].second = true;
    pns->stopTimer(SENDER, ack_packet.acknum);
    pUtils->printPacket("[SR Sender]: ACK packet received successfully.",
                        ack_packet);
    fprintf(stdout, "[SR Sender]: Window status:\n[ ");
    for (int i = 0; i < window.size(); i++) {
      if (window[i].second) {
        fprintf(stdout, "%dT ", (sendbase + i) % this->seq_len);
      } else {
        fprintf(stdout, "%dF ", (sendbase + i) % this->seq_len);
      }
    }
    while (!window.empty() && window.front().second) {
      sendbase = (sendbase + 1) % seq_len;
      window.pop_front();
    }
    fprintf(stdout, "] =>\n[ ");
    for (int i = 0; i < window.size(); i++) {
      if (window[i].second) {
        fprintf(stdout, "%dT ", (sendbase + i) % this->seq_len);
      } else {
        fprintf(stdout, "%dF ", (sendbase + i) % this->seq_len);
      }
    }
    fprintf(stdout, "]\n");
  } else {
    pUtils->printPacket("[SR Sender]: ACK packet received corrupted.",
                        ack_packet);
  }
}

void SRRdtSender::timeoutHandler(int seqNum) {
  int window_idx = (seqNum - sendbase + seq_len) % seq_len;
  pns->stopTimer(SENDER, seqNum);
  pns->startTimer(SENDER, Configuration::TIME_OUT, seqNum);
  pns->sendToNetworkLayer(RECEIVER, window[window_idx].first);
  pUtils->printPacket("[SR Sender]: timeouts, resending the packet.",
                      window[window_idx].first);
}
