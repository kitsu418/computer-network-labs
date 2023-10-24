#ifndef GBN_RDT_RECEIVER_H
#define GBN_RDT_RECEIVER_H

#include "RdtReceiver.h"

class GBNRdtReceiver : public RdtReceiver {
private:
  int nextseqnum;
  Packet last_ack_packet;
  int seq_len;

public:
  GBNRdtReceiver();
  virtual ~GBNRdtReceiver();

public:
  void receive(const Packet &packet);
};

#endif
