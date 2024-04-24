## Muhammad Haekal Kalipaksi

## 2206817490

### Reflections

**What is amqp?**
AMQP is Advance Message Queue Protocol is a open standard protocol for messages passing and queue between different application or services. In this tutorial we use RabbtitMq messages broker which implement AMQP.

Amqp is

**what it means? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?**

First guest is username, second guest is password and, the localhost:5672 is where the queue server located. Im using .env to store the amqp url.

**Simulation slow subscriber**

![slow-subcriber](/images/slow.png)

In my case the queue is below 20 but above 15, so its around that, because of the delay from thread 10 millis second, subscriber will receive the messages slower and rabbitmq must keep the messages send by publisher into the queue and send the messages to subscriber until the queue is empty. In images above the messages seding in rate is 3.0/second and 1.0/second
