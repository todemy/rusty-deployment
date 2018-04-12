class Tody < Formula
    desc ""
    homepage "https://github.com/todemy/tody-cli"
    url "https://github.com/todemy/tody-cli/blob/master/homebrew/Release/tody-0.1.0.tar.gz?raw=true"
    sha256 "76d8cbd1cf09c5a43e102b3f2732a9522f9a54c75aeb3a4a2f680bbd4490c2c2"
  
    def install
      bin.install "tody"
    end
  end