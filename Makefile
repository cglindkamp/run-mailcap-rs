PROJECT=run-mailcap-rs
PREFIX=/usr/local
BINDIR=$(PREFIX)/bin
LINKS=see edit compose print
COMPAT_LINKS=0

BINARY=target/release/$(PROJECT)
all: $(BINARY)

$(BINARY):
	cargo build --release

install: $(BINARY)
	install -m 755 -D $(BINARY) $(BINDIR)/$(PROJECT)
	for LINK in $(LINKS); do \
		ln -sf $(PROJECT) $(BINDIR)/$$LINK""-rs; \
	done
	if [ $(COMPAT_LINKS) -eq 1 ]; then \
		for LINK in $(LINKS); do \
			ln -sf $(PROJECT) $(BINDIR)/$$LINK; \
		done; \
		ln -sf $(PROJECT) $(BINDIR)/run-mailcap; \
	fi

uninstall:
	rm -f $(BINDIR)/$(PROJECT)
	for LINK in $(LINKS); do \
		rm -f $(BINDIR)/$$LINK""-rs; \
	done
	if [ $(COMPAT_LINKS) -eq 1 ]; then \
		for LINK in $(LINKS); do \
			rm -f $(BINDIR)/$$LINK; \
		done; \
		rm -f $(BINDIR)/run-mailcap; \
	fi
