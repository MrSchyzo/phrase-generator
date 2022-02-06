#!/bin/bash

if [ -z "$(docker images | grep "quarkus/tts-rest-wrapper" | grep -v "jvm")" ]; then
  >&2 echo "Seems you do not have any image named quarkus/tts-rest-wrapper... creating one automatically!";

  tmp_dir=$(mktemp -d -t ci-XXXXXXXXXX);

  >&2 echo -n "Temporary clone of tts-rest-wrapper in temp dir $tmp_dir... "
  pushd "$tmp_dir" || exit 1;


  if git clone https://github.com/MrSchyzo/tts-rest-wrapper.git >/dev/null 2>&1 ; then
    echo "✔";
  else
    echo "❌"; exit 1;
  fi

  cd tts-rest-wrapper || exit 1;
  >&2 echo "Generating docker-compose for tts-rest-wrapper $tmp_dir... ";
  >&2 ./generate-replicas-local-deploy.sh;

  popd && rm -rf "$tmp_dir";
fi


>&2 echo "Composing docker images for phrasegen..."
docker stop dev-pgql >/dev/null 2>&1
echo "Activating development postgres" && \
docker run --name dev-pgql --rm -p 12345:5432 -e POSTGRES_PASSWORD=postgres -d postgres:14.1 && \
sleep 3 && \
docker-compose build && \
docker-compose up --detach && \
docker stop dev-pgql && \
>&2 echo "Done! The following services are available now:" && \
>&2 echo "- phrasegen playground: http://localhost:81" && \
>&2 echo "- phrasegen health: http://localhost:81/health" && \
>&2 echo "- tts-wrapper: http://localhost[/speak]" && \
>&2 echo "- postgresql: postgres://localhost:54321"
