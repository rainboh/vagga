#!/bin/sh -ex
ALPINE_VERSION=v3.7
ALPINE_MIRROR=http://dl-cdn.alpinelinux.org/alpine/
APK_TOOLS=apk-tools-static-2.8.1-r2.apk
BUSYBOX=busybox-static-1.27.2-r7.apk
ALPINE_KEYS=alpine-keys-2.1-r1.apk

ARCH=${1:-x86_64}

SHA1SUMS_x86_64="\
c37090cfdf892f052e55952244559e38f13809ea  $APK_TOOLS
eff85e3a544ca147c24e879172603a038d0204a5  $BUSYBOX
9e1f9418c83dbaa95397ede62781e4793e30a959  $ALPINE_KEYS"

SHA1SUMS_armhf="\
42eb95f35b44102caee8d5fb6ea5ace3a43f7588  $APK_TOOLS
eec73d81dfa78d3440e6cc02885600da4296da1d  $BUSYBOX
d752cec62c2944fc6fa6aff4e533e4ee2ae1a4a0  $ALPINE_KEYS"

FETCH_DIR="alpine/"$ARCH
mkdir -p "$FETCH_DIR" 2>/dev/null || true
cd "$FETCH_DIR"

for pkg in $APK_TOOLS $BUSYBOX $ALPINE_KEYS; do
    wget --no-use-server-timestamp ${ALPINE_MIRROR}${ALPINE_VERSION}/main/$ARCH/$pkg -O $pkg
done

sha1sum $APK_TOOLS
sha1sum $BUSYBOX
sha1sum $ALPINE_KEYS
SUMS="SHA1SUMS_$ARCH"
eval "SUMS=\$$SUMS"
echo "$SUMS" | sha1sum -c -

cd ../..

tar -xOf "$FETCH_DIR/$APK_TOOLS" sbin/apk.static > apk
tar -xOf "$FETCH_DIR/$BUSYBOX" bin/busybox.static > busybox
cp "$FETCH_DIR/$ALPINE_KEYS" alpine-keys.apk

chmod +x apk busybox
