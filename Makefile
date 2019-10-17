test:
	wasm-pack test --headless --chrome --release

install:
	brew cask install chromewebdriver
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 