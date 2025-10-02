<?php

$IMAGE = "stampy";
$CONTAINER="stampy-container";

echo implode("\n",[
'tmp_doc=tmp-link-$$-$(date +%s)',
'mkdir -p $tmp_doc',
"docker buildx build --progress=plain -f ./vendor/stampy/php-cli/docker-bin/dockerfile.stampy ./ -t $IMAGE:latest --platform linux/arm64",
'rm -rf $tmp_doc',
"docker run -it --platform linux/arm64 --name $CONTAINER $IMAGE:latest /bin/sh -c \"",
"\tCOMPOSER=composer.json php -a -d extension=/usr/lib/php/20240924/stampy_php_cli.so docker-version/install.php > var.txt",
"\"",
]);