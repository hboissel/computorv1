BUILD	:= debug

CARGOFLAGS.debug	:=
CARGOFLAGS.release	:= --release

all:
	cargo build ${CARGOFLAGS.${BUILD}}
	cp target/${BUILD}/computorv1 computorv1

clean:
	cargo clean
	rm -f computorv1

re: clean all

test:
	cargo test