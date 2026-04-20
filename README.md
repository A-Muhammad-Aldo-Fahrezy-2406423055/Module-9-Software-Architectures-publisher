# Understanding Publisher and Message Broker

## a. Berapa banyak data yang dikirim publisher ke message broker dalam satu kali run?

Dalam satu kali eksekusi (`cargo run`), program publisher mengirimkan **5 event** ke *message broker*. Masing-masing event berupa objek `UserCreatedEventMessage` yang memiliki dua field: `user_id` dan `user_name`. Kelima pesan tersebut adalah:

| user_id | user_name |
|---------|-----------|
| 1 | 2406423055-Amir |
| 2 | 2406423055-Budi |
| 3 | 2406423055-Cica |
| 4 | 2406423055-Dira |
| 5 | 2406423055-Emir |

Setiap pesan diserialkan menggunakan *Borsh serialization* sebelum dikirim ke queue bernama `user_created` pada RabbitMQ.

## b. URL `amqp://guest:guest@localhost:5672` sama dengan subscriber, apa artinya?

Penggunaan URL yang **sama persis** antara publisher dan subscriber menunjukkan bahwa keduanya terhubung ke **message broker yang sama**, yaitu instansi RabbitMQ yang berjalan di `localhost:5672`. Ini adalah inti dari arsitektur *event-driven* dimana publisher dan subscriber tidak berkomunikasi secara langsung satu sama lain, melainkan keduanya dihubungkan melalui perantara (*broker*) yang sama. Publisher cukup mengirim pesan ke broker dan broker yang bertanggung jawab meneruskannya ke subscriber yang mendaftarkan diri pada queue yang sesuai (`user_created`). Dengan demikian, publisher dan subscriber tetap *loosely coupled* (mereka tidak perlu saling mengenal).

# Screenshot of running RabbitMQ
![Running RabbitMQ](assets/images/RabbitMQ.png)

# Sending and Processing Event

### Screenshot of RabbitMQ Connection
![RabbitMQ Connection](assets/images/RabbitMQ%20connections.png)

Setelah subscriber dijalankan dengan `cargo run`, tampilan RabbitMQ Management di
`http://localhost:15672/#/connections` menunjukkan **1 koneksi aktif** dari subscriber
(IP `192.168.65.1` dengan port `28719`) menggunakan protokol AMQP 0-9-1 dengan
status **running**. Hal ini membuktikan bahwa subscriber telah berhasil terhubung ke
message broker dan siap menerima event dari queue `user_created`.

---

### Screenshot of Publisher & Subscriber Console
![Publisher Subscriber Console](assets/images/Publisher-Subscriber%20console.png)

Ketika `cargo run` dijalankan pada direktori publisher, publisher mengirimkan **5 event**
sekaligus ke message broker RabbitMQ pada queue `user_created`. Kelima event tersebut
berisi data `UserCreatedEventMessage` dengan `user_id` 1–5 dan `user_name`
`2406423055-Amir` hingga `2406423055-Emir`.

Event-event tersebut tidak langsung dikirim ke subscriber, melainkan terlebih dahulu
masuk ke antrian (*queue*) di RabbitMQ. Subscriber yang sudah berjalan dan mendengarkan
queue `user_created` kemudian mengambil dan memproses setiap event satu per satu,
mencetak output seperti: