# Imagem oficial enxuta
FROM postgres:16-alpine

# Scripts de inicialização (executam só no 1º start do volume)
COPY --chmod=0755 init/*.sh /docker-entrypoint-initdb.d/
