#include "SRRdtReceiver.h"
#include "DataStructure.h"
#include "Global.h"
#include <cstdio>
#include <utility>

SRRdtReceiver::SRRdtReceiver()
    : recvbase(0), seq_len(WINDOW_LENGTH * 2), window_len(WINDOW_LENGTH),
      window({}) {
  last_ack_packet.acknum = -1;
  last_ack_packet.checksum = 0;
  last_ack_packet.seqnum = -1;
  for (int i = 0; i < Configuration::PAYLOAD_SIZE; i++) {
    last_ack_packet.payload[i] = '.';
  }
  last_ack_packet.checksum = pUtils->calculateCheckSum(last_ack_packet);
  for (int i = 0; i < window_len; ++i) {
    Packet blank{};
    blank.seqnum = -1;
    window.push_back(std::make_pair(blank, false));
  }
}

SRRdtReceiver::~SRRdtReceiver() {}

void SRRdtReceiver::receive(const Packet &packet) {
  int checksum = pUtils->calculateCheckSum(packet);
  int window_idx = (packet.seqnum - recvbase + seq_len) % seq_len;

  if (checksum == packet.checksum && window_idx < window_len &&
      window[window_idx].second == false) {
    pUtils->printPacket("[SR Receiver]: packet received successfully.", packet);
    fprintf(stdout, "[SR Receiver]: Window status:\n[ ");
    for (int i = 0; i < window.size(); i++) {
      if (window[i].second) {
        fprintf(stdout, "%dT ", (recvbase + i) % this->seq_len);
      } else {
        fprintf(stdout, "%dF ", (recvbase + i) % this->seq_len);
      }
    }
    fprintf(stdout, "]\n ");
    window[window_idx].first = packet;
    window[window_idx].second = true;

    while (window.front().second) {
      Message msg;
      memcpy(msg.data, window.front().first.payload, sizeof(window.front().first.payload));
      pns->delivertoAppLayer(RECEIVER, msg);
      recvbase = (recvbase + 1) % seq_len;
      Packet blank = {};
      blank.seqnum = -1;
      window.pop_front();
      window.push_back(std::make_pair(blank, false));
    }
    fprintf(stdout, "=>\n[ ");
    for (int i = 0; i < window.size(); i++) {
      if (window[i].second) {
        fprintf(stdout, "%dT ", (recvbase + i) % this->seq_len);
      } else {
        fprintf(stdout, "%dF ", (recvbase + i) % this->seq_len);
      }
    }
    fprintf(stdout, "]\n");
    last_ack_packet.acknum = packet.seqnum;
    last_ack_packet.checksum = pUtils->calculateCheckSum(last_ack_packet);
    pUtils->printPacket("[SR Receiver]: sent ACK packet.", last_ack_packet);
    pns->sendToNetworkLayer(SENDER, last_ack_packet);
  } else {
    if (checksum != packet.checksum) {
      pUtils->printPacket("[SR Receiver]: packet received corrupted.", packet);
    } else {
      pUtils->printPacket("[SR Receiver]: packet has been received before.",
                          packet);
      last_ack_packet.acknum = packet.seqnum;
      last_ack_packet.checksum = pUtils->calculateCheckSum(last_ack_packet);
      pUtils->printPacket("[SR Receiver]: resending last ACK packet.",
                          last_ack_packet);
      pns->sendToNetworkLayer(SENDER, last_ack_packet);
    }
  }
}