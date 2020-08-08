(function() {var implementors = {};
implementors["either"] = [{"text":"impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,&nbsp;</span>","synthetic":false,"types":["either::Either"]}];
implementors["image"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"image/dxt/struct.DXTReader.html\" title=\"struct image::dxt::DXTReader\">DXTReader</a>&lt;R&gt;","synthetic":false,"types":["image::dxt::DXTReader"]},{"text":"impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"image/gif/struct.GifReader.html\" title=\"struct image::gif::GifReader\">GifReader</a>&lt;R&gt;","synthetic":false,"types":["image::gif::GifReader"]},{"text":"impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"image/hdr/struct.HdrReader.html\" title=\"struct image::hdr::HdrReader\">HdrReader</a>&lt;R&gt;","synthetic":false,"types":["image::hdr::decoder::HdrReader"]},{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"image/png/struct.PNGReader.html\" title=\"struct image::png::PNGReader\">PNGReader</a>&lt;R&gt;","synthetic":false,"types":["image::png::PNGReader"]},{"text":"impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"image/tiff/struct.TiffReader.html\" title=\"struct image::tiff::TiffReader\">TiffReader</a>&lt;R&gt;","synthetic":false,"types":["image::tiff::TiffReader"]}];
implementors["inflate"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"inflate/struct.DeflateDecoderBuf.html\" title=\"struct inflate::DeflateDecoderBuf\">DeflateDecoderBuf</a>&lt;R&gt;","synthetic":false,"types":["inflate::reader::DeflateDecoderBuf"]},{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"inflate/struct.DeflateDecoder.html\" title=\"struct inflate::DeflateDecoder\">DeflateDecoder</a>&lt;R&gt;","synthetic":false,"types":["inflate::reader::DeflateDecoder"]}];
implementors["rand_core"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for dyn <a class=\"trait\" href=\"rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>","synthetic":false,"types":["rand_core::RngCore"]}];
implementors["rmp_serde"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"rmp_serde/decode/struct.ReadReader.html\" title=\"struct rmp_serde::decode::ReadReader\">ReadReader</a>&lt;R&gt;","synthetic":false,"types":["rmp_serde::decode::ReadReader"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"rmp_serde/decode/struct.ReadRefReader.html\" title=\"struct rmp_serde::decode::ReadRefReader\">ReadRefReader</a>&lt;'a, T&gt;","synthetic":false,"types":["rmp_serde::decode::ReadRefReader"]}];
implementors["smithay_client_toolkit"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for <a class=\"struct\" href=\"smithay_client_toolkit/data_device/struct.ReadPipe.html\" title=\"struct smithay_client_toolkit::data_device::ReadPipe\">ReadPipe</a>","synthetic":false,"types":["smithay_client_toolkit::data_device::data_offer::ReadPipe"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()