<?php
$stream = json_decode(stream_get_contents(STDIN));
$entry = dirname(__DIR__,2)."/test/console/bin";

echo implode("\n",[
    "#!/bin/bash",
    "export $(grep -v '^#' .env | xargs)",
    ' ext="extension=vendor/stampy/php-cli/target/release/libstampy_php_cli.dylib" ',
    "if [ $# -eq 0 ]; then",
    "\t".'php -d $ext ' . $entry,
    "\texit 1",
    'fi',
    'args=""',
    'for arg in "$@";do',
    "\t".' args="$args $arg"',
    'done',
    'php -d $ext '. $entry . ' $args'
]);

