FROM rust:latest

# Loyihani muallifi
LABEL authors="jusho"

# Ishchi katalogni yaratish va unda ishlash
WORKDIR /app

# Cargo.toml va Cargo.lock fayllarini ko'chirish
COPY Cargo.toml Cargo.lock ./

# Paketlarni yuklash
RUN cargo fetch

# Barcha fayllarni ko'chirish
COPY . .

# Loyihani build qilish
RUN cargo build --release

# ENTRYPOINT - dastur qaysi komandadan boshlanishi kerak
ENTRYPOINT ["./target/release/telegram-bot-boshlangish"]
