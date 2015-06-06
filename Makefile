.PHONY: all
all: proto test

.PHONY: proto
proto:
	protoc --proto_path=cockroach-proto --rust_out=./src cockroach-proto/cockroach/proto/*.proto

.PHONY: test
test:
	cargo test

.PHONY: dockertest
dockertest:
	-rm -f target/debug/rustroach
	cargo test --no-run
	(cd target/debug && ln -s rustroach-* rustroach)
	-docker-compose stop
	-docker-compose rm --force -v cockroach
	-rm -rf /tmp/test-disk1
	mkdir /tmp/test-disk1
	docker-compose build
	# TODO: ditch the --insecure; client needs to learn TLS for that.
	# docker-compose run cockroach cert create-ca --certs=/data
	# docker-compose run cockroach cert create-node --certs=/data localhost 127.0.0.1 $(hostname)
	docker-compose run cockroach init --stores=hdd=/data
	docker-compose start cockroach
	docker-compose run rusttest
	docker-compose stop --timeout 0
