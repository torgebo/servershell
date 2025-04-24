DOC_RUN="run: \t\tlaunch server with client connection in debug mode"
.PHONY: run
run:
	RUST_LOG=tower_http=trace \
	MARSWEATHER_ENDPOINT='https://mars.nasa.gov/rss/api/?feed=weather&feedtype=json&ver=1.0&category=msl' \
			     cargo run


DOC_TEST="test:\t\tstatic and runtime checks"
.PHONY: test
test:
	cargo test
	cargo fmt
	cargo clippy


DOC_DENY="deny:\t\tcheck licences, vulnerabilities, etc"
.PHONY: deny
deny:
	cargo deny check


DOC_DOCKER_BUILD="docker_build:\tbuild docker app"
.PHONY: docker_build
docker_build:
	docker build --tag servershell:0.1.0 .


DOC_DOCKER_RUN="docker_run: \trun docker app"
.PHONY: docker_run
docker_run: docker_build
	docker run \
	--network host \
	--env-file .env-file \
	servershell:0.1.0


DOC_EXAMPLE_QUERY="example_query:\trun against live server to observe response contents and headers"
.PHONY: example_query
example_query:
	curl -v http://localhost:3000/marsweather?date=2022-07-11


.PHONY: help
help:
	@echo ${DOC_RUN}
	@echo ${DOC_TEST}
	@echo ${DOC_DENY}
	@echo ${DOC_DOCKER_BUILD}
	@echo ${DOC_DOCKER_RUN}
	@echo ${DOC_EXAMPLE_QUERY}

