#include "TCPRdtReceiver.h"
#include "Global.h"

TCPRdtReceiver::TCPRdtReceiver() : nextseqnum(0), seq_len(WINDOW_LENGTH * 2) {
  last_ack_packet.acknum = -1;
  last_ack_packet.checksum = 0;
  last_ack_packet.seqnum = -1;
  for (int i = 0; i < Configuration::PAYLOAD_SIZE; i++) {
    last_ack_packet.payload[i] = '.';
  }
  last_ack_packet.checksum = pUtils->calculateCheckSum(last_ack_packet);
}

TCPRdtReceiver::~TCPRdtReceiver() {}

void TCPRdtReceiver::receive(const Packet &packet) {
  int checksum = pUtils->calculateCheckSum(packet);

  if (checksum == packet.checksum && nextseqnum == packet.seqnum) {
    pUtils->printPacket("[TCP Receiver]: packet received successfully.",
                        packet);

    Message msg;
    memcpy(msg.data, packet.payload, sizeof(packet.payload));
    pns->delivertoAppLayer(RECEIVER, msg);

    last_ack_packet.acknum = packet.seqnum;
    last_ack_packet.checksum = pUtils->calculateCheckSum(last_ack_packet);
    pUtils->printPacket("[TCP Receiver]: sent ACK packet.", last_ack_packet);
    pns->sendToNetworkLayer(SENDER, last_ack_packet);
    nextseqnum = (nextseqnum + 1) % seq_len;
  } else {
    if (checksum != packet.checksum) {
      pUtils->printPacket("[TCP Receiver]: packet received corrupted.", packet);
    } else {
      pUtils->printPacket("[TCP Receiver]: incorrect packet seqnum.", packet);
    }
    pUtils->printPacket("[TCP Receiver]: resending last ACK packet.",
                        last_ack_packet);
    pns->sendToNetworkLayer(SENDER, last_ack_packet);
  }
}