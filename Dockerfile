FROM node:current-alpine

COPY index.js /app/
WORKDIR /app
CMD node index.js