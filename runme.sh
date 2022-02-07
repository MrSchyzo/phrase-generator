#!/bin/bash

if [ -z "$(docker images | grep "quarkus/tts-rest-wrapper" | grep -v "jvm")" ]; then
  >&2 echo "Seems you do not have any image named quarkus/tts-rest-wrapper... creating one automatically!";

  # https://stackoverflow.com/a/56243046
  java=$(java -version 2>&1 | head -1 | cut -d'"' -f2 | sed '/^1\./s///' | cut -d'.' -f1)
  docker=$(docker version --format '{{.Server.Version}}')

  if [[ "${java:-1}" -lt 11 ]] ; then
    >&2 echo "Failed. You need at least java 11 to run this."
    exit 1;
  fi

  if [[ "${docker:-00.00}" < '19.03' ]] ; then
    >&2 echo "Failed. You need at least docker 19.03 to run this."
    exit 1;
  fi

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
sed -e 's/##USER##/'"$(id -u)"'/g' -e 's/##GROUP##/'"$(id -g)"'/g' docker-compose-template.yml > docker-compose.yml && \
docker-compose build && \
docker-compose up --detach && \
docker stop dev-pgql && \
>&2 echo "Done! The following services are available now:" && \
>&2 echo "- phrasegen playground: http://localhost:81" && \
>&2 echo "- phrasegen health: http://localhost:81/health" && \
>&2 echo "- tts-wrapper: http://localhost[/speak]" && \
>&2 echo "- postgresql: postgres://localhost:54321"
