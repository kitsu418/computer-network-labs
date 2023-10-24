#ifdef GBN
#include "GBNRdtReceiver.h"
#include "GBNRdtSender.h"
#elif SR
#include "SRRdtReceiver.h"
#include "SRRdtSender.h"
#elif TCP
#include "TCPRdtReceiver.h"
#include "TCPRdtSender.h"
#endif

#include "Global.h"
#include "RdtReceiver.h"
#include "RdtSender.h"
#include <cstdio>

int main(int argc, char *argv[]) {
  if (argc != 3) {
    printf("Usage: command /path/to/input/file /path/to/output/file");
    return -1;
  }

#ifdef GBN
  RdtSender *ps = new GBNRdtSender();
  RdtReceiver *pr = new GBNRdtReceiver();
#elif SR
  RdtSender *ps = new SRRdtSender();
  RdtReceiver *pr = new SRRdtReceiver();
#elif TCP
  RdtSender *ps = new TCPRdtSender();
  RdtReceiver *pr = new TCPRdtReceiver();
#endif
  pns->setRunMode(0);
  // pns->setRunMode(1);
  pns->init();
  pns->setRtdSender(ps);
  pns->setRtdReceiver(pr);
  pns->setInputFile(argv[1]);
  pns->setOutputFile(argv[2]);

  pns->start();

  delete ps;
  delete pr;
  delete pUtils;
  delete pns;

  return 0;
}
