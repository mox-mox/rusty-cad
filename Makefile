report_error = || (retval=$$?; notify-send -t 5000 -u critical "Build for $@ failed"; exit $$retval)
FILE:=bett
all: target/debug/$(FILE).scad

target/debug/$(FILE).scad: target/debug/rusty_scad
	./$< > $@

target/debug/rusty_scad: src/*.rs
	cargo build

.PHONY: clean
clean:
	rm -rf target target/debug/$(FILE).scad

.PHONY: view
view: target/debug/$(FILE).scad
	@echo "Starting openscad"
	@bash -c "openscad $^ 2>/dev/null > /dev/null &" $(report_error)
	#@bash -c "openscad $^&"



