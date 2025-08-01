<?php
$stream = json_decode(stream_get_contents(STDIN));

echo 'ENTRY="'. ($stream?->{"stampy-config"}?->{"entry"}.'"') ?? 'add entry..."';
echo PHP_EOL;
echo 'NAMESPACE="'. ($stream?->{"stampy-config"}?->{"namespace"}.'"') ?? 'add namespace..."';
