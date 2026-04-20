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