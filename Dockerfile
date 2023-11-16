FROM rust:1.67
LABEL authors="machado"

WORKDIR /usr/src/myapp
COPY . .

CMD ["bash"]
