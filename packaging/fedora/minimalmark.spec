Name:           minimalmark
Version:        0.1.0
Release:        1%{?dist}
Summary:        A minimal, fast Markdown editor for GNOME

License:        MIT
URL:            https://github.com/minimalmark/minimalmark
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  gtk4-devel
BuildRequires:  glib2-devel
BuildRequires:  webkit2gtk4.1-devel
BuildRequires:  libadwaita-devel
BuildRequires:  gtksourceview5-devel

%description
MinimalMark is a lightweight, fast Markdown editor designed for the GNOME desktop.
It provides a clean, distraction-free writing experience with live preview.

Features:
- Split-pane editing with live preview
- Full CommonMark + GFM support (tables, task lists, strikethrough)
- Syntax highlighting in editor
- Live word count and reading time
- Dark/light/system themes
- Formatting toolbar with keyboard shortcuts
- Auto-save and crash recovery
- Export to HTML and PDF
- Spell checking
- Focus mode
- macOS-inspired design with rounded corners

%install
cargo build --release --target-dir .

install -Dm755 target/release/minimalmark %{buildroot}%{_bindir}/minimalmark
install -Dm644 data/io.github.minimalmark.desktop %{buildroot}%{_datadir}/applications/io.github.minimalmark.desktop
install -Dm644 data/io.github.minimalmark.metainfo.xml %{buildroot}%{_datadir}/metainfo/io.github.minimalmark.metainfo.xml
install -Dm644 data/style.css %{buildroot}%{_datadir}/minimalmark/style.css
install -Dm644 data/icons/hicolor/scalable/apps/io.github.minimalmark.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/io.github.minimalmark.svg

%files
%{_bindir}/minimalmark
%{_datadir}/applications/io.github.minimalmark.desktop
%{_datadir}/metainfo/io.github.minimalmark.metainfo.xml
%{_datadir}/minimalmark/style.css
%{_datadir}/icons/hicolor/scalable/apps/io.github.minimalmark.svg

%changelog
* Sat May 16 2026 - Initial release
