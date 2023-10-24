#ifndef TCP_RDT_RECEIVER_H
#define TCP_RDT_RECEIVER_H

#include "RdtReceiver.h"

class TCPRdtReceiver : public RdtReceiver {
private:
  int nextseqnum;
  Packet last_ack_packet;
  int seq_len;

public:
  TCPRdtReceiver();
  virtual ~TCPRdtReceiver();

public:
  void receive(const Packet &packet);
};

#endif
