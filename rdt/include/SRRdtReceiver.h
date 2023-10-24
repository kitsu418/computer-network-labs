#ifndef SR_RDT_RECEIVER_H
#define SR_RDT_RECEIVER_H

#include "DataStructure.h"
#include "RdtReceiver.h"
#include <deque>
#include <utility>

class SRRdtReceiver : public RdtReceiver {
private:
  Packet last_ack_packet;
  int recvbase;
  int seq_len;
  int window_len;
  std::deque<std::pair<Packet, bool>> window;

public:
  SRRdtReceiver();
  virtual ~SRRdtReceiver();

public:
  void receive(const Packet &packet);
};

#endif
