class Tailspin < Formula
  desc "Log file highlighter"
  homepage "https://github.com/bensadeh/tailspin"
  url "https://github.com/bensadeh/tailspin/archive/refs/tags/1.6.1.tar.gz"
  sha256 "244163902523c9350658dca6b9e74aaddeb7635bd9195e21f8cfde0b62844e8e"
  license "MIT"

  livecheck do
    url "https://github.com/bensadeh/tailspin/releases/tag/"
    regex(/href=.*?tailspin[._-]v?(\d+(?:\.\d+)+)\.t/i)
  end

  depends_on "rust" => :build

  conflicts_with "spin", because: "spin also ships a spin binary"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system bin/"spin", "--help"
  end
end
