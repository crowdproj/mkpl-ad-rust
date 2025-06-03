FROM swaggerapi/swagger-ui:latest

RUN rm -f /docker-entrypoint.d/40-swagger-ui.sh
COPY ./build-swagger/swagger-initializer.js /usr/share/nginx/html/swagger-initializer.js
COPY ./specs/* /usr/share/nginx/html/specs/

ENV SWAGGER_JSON=/swagger-initializer.js
