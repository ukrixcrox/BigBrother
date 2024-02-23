# BigBrother
a Network intrusion prevention system (NIPS)

# IDEA  
- the IDS has two modes:
    1. Watch and Collect: Scannes and collects info from malicious ip addresses
    2. Serve and Protect: "Kicks" malicious ip addresses  

- Detection method:
    1. Signature-based
    2. Statistical Anomaly-based
    3. Stateful protocol analysis

- looks at all send packages

# Parts
 - webserver/webinterface service
 - messaging service (handles all the messaging/Email/telegram/whatsapp/irc/matrix/whatever)
 - NIPS service

# TODO
- handle all incoming packages
