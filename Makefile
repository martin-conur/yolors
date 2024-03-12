format:
	cargo fmt 

lint: 
	cargo clippy 
	
prepare:
	make format lint

run:
	cargo run --release demo_images/people.jpg