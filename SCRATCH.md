
## Notes

kafkacat -L -b kafka:9092

mysql.inventory.products
mysql.inventory.user
mysql.inventory.region
mysql.inventory.addresses
mysql.simple.purchase
mysql.inventory.customers


kafkacat -C -b kafka:9092 -t mysql.inventory.customers -s value=avro -r http://schema-registry:8080 -o -1 -e

kafkacat -C -b kafka:9092 -t mysql.inventory.products -J  -o -1 -e

kafkacat -C -b kafka:9092 -t mysql-history -J  -o -1 -e

{"topic":"mysql.inventory.customers","partition":0,"offset":3,"key":"\u0000\u0000\u0000\u0000\u0005?\u000F","payload":"\u0000\u0000\u0000\u0000\u0006\u0000\u0002?\u000F\bAnne\u0012Kretchmar$annek@noanswer.org\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0012customers\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002???܎\\"}

AnneKretchmar$annek@noanswer.org1.0.1.Final
mysql
mysqtrueinventorycustomers mysql-bin.000003??$c???܎\



```
{"topic":"mysql.inventory.products","partition":0,"offset":8,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\u0014spare tire\u0002$24 inch spare tire\u0002\u0000\u0000\u0000@336@\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ʁ?܎\\"}
% Reached end of topic mysql.inventory.products [0] at offset 9: exiting
root@ba86386524d1:/# kafkacat -C -b kafka:9092 -t mysql.inventory.products -J  -o -10 -e
{"topic":"mysql.inventory.products","partition":0,"offset":0,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\u000Escooter\u0002*Small 2-wheel scooter\u0002\u0000\u0000\u0000`?\u001E\t@\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002Ɓ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":1,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\u0016car battery\u0002\u001E12V car battery\u0002\u0000\u0000\u0000@33 @\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ȁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":2,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001$12-pack drill bits\u0002n12-pack of drill bits with sizes ranging from #40 to #3\u0002\u0000\u0000\u0000?????\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ȁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":3,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\fhammer\u0002.12oz carpenter's hammer\u0002\u0000\u0000\u0000\u0000\u0000\u0000??\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ȁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":4,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\fhammer\u0002.14oz carpenter's hammer\u0002\u0000\u0000\u0000\u0000\u0000\u0000??\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ȁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":5,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\fhammer\u0002.16oz carpenter's hammer\u0002\u0000\u0000\u0000\u0000\u0000\u0000??\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ȁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":6,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\nrocks\u0002*box of assorted rocks\u0002\u0000\u0000\u0000@33\u0015@\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ʁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":7,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\fjacket\u0002Dwater resistent black wind breaker\u0002\u0000\u0000\u0000?????\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ʁ?܎\\"}
{"topic":"mysql.inventory.products","partition":0,"offset":8,"key":"\u0000\u0000\u0000\u0000\u000B?\u0001","payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\u0014spare tire\u0002$24 inch spare tire\u0002\u0000\u0000\u0000@336@\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ʁ?܎\\"}
% Reached end of topic mysql.inventory.products [0] at offset 9: exiting
```

```
{"topic":"mysql.inventory.products",
"partition":0,
"offset":6,
"key":"\u0000\u0000\u0000\u0000\u000B?\u0001",
"payload":"\u0000\u0000\u0000\u0000\f\u0000\u0002?\u0001\nrocks\u0002*box of assorted rocks\u0002\u0000\u0000\u0000@33\u0015@\u00161.0.1.Final\nmysql\nmysql\u0000\u0000\btrue\u0012inventory\u0002\u0010products\u0000\u0000 mysql-bin.000003??$\u0000\u0000\u0000\u0002c\u0002ʁ?܎\\"}
```