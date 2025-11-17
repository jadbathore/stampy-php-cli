# Stampy/php-cli

Is a composer plugin that makes it easier to use the PHP command prompt in your project in a clean and organized way.

## installation

> [!TIP]
> "During installation, Composer will probably prompt you to authorize the plugin to run. Please answer 'yes' to this message."

To install this project using Composer, run:

```bash
composer require stampy/php-cli
```


> [!IMPORTANT]
> Stampy/php-cli is a Composer plugin. You must allow it to run by approving the prompt during installation. If you answer 'y', Composer will automatically add the permission to your composer.json file. If you answer 'no', you will need to add the following setting to your composer.json file manually:

``` json
"config": {
        "allow-plugins": {
            "stampy/php-cli": true
        }
    },
```
After installation, the plugin will check if a pre-compiled version of the Stampy library is available for your system.
### If a pre-compiled version **exists** for your architecture

Then you would probably see this message

```md
Generating autoload files
? the stampy extension add already a pre-compile binairy you can  ?  ›
  continue as such
  use docker
```
This confirms that the pre-compiled library is compatible with your architecture. You can now choose to continue with the native installation or use Docker.**For your convenience, an executable Bash script has been installed. This script allows you to launch the command prompt easily later, even if you use the pre-compiled library now.**

If your project uses multiple namespaces, Stampy will prompt you to select which namespace should contain the controller for the plugin.

It will also create a `.env` file at the root of your Composer project. You can use this file to define environment variables that are specific to the plugin.

Finally, an executable file named `stampy` will be created at the root of your project. You can use this file to run the command prompt.

> [!CAUTION]
> Do not remove the $NAMESPACE and $ENTRY variables from your `.env` file. They are essential for the plugin to function properly.
> You can modify these variables if you wish to run the plugin from a different location.

### If there **doesn't exist** a precompiled version for your architecture
If the plugin fails to find a compatible pre-compiled version, it will display this message:
```md
the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself
using cargo or use docker.If you using cargo make sure you got cargo install (https://rust-lang.org/tools/install/).
If you using docker make sure you docker daemon running [cargo|docker] ?
```
No problem! Stampy will use Cargo to compile the library and create a custom executable for your architecture.

Please note that because the library relies on various Cargo extensions, it may not be compatible with all system architectures. If this is the case for your system, we recommend using the containerized (Docker) version of the plugin instead.

### Docker 

To use Docker, simply select the "[use docker]" option during installation. This is also the recommended choice if your system lacks a pre-compiler.

Alternatively, a Bash script is created during installation. You can run the following command at any time to containerize the plugin:

```bash
  composer dockerStampy
```

> [!TIP]
> Before running the command, make sure you're at the root of your Composer project, i.e., the directory that contains the composer.json file.

By following this installation process, you will achieve the same result as with the other methods: the terminal will prompt you to select a namespace for Stampy.

It will then create several files in your application directory:
- A controller
- An .env file
- A composer.stampy.json file

This separate composer.stampy.json file exists because the Docker version of Stampy operates independently from your main application. This complete separation ensures that the plugin's dependencies and capabilities do not interfere with those of your application.

> [!CAUTION]
> Be careful not to give your new controller the same name as an existing one, as this will overwrite it.

## Composer.json post-install

After installing the plugin, your composer.json file will have changed. First, a new namespace will be added to it.
```json
  "autoload": {
        "psr-4": {
            "App\\": "src/",
            "StampyConsole\\": "src/console/"
        }
    },
```
It is within this namespace that you will use the controllers to add elements for building your command prompt. By default, this namespace is also set in your .env file.

You will also notice that the $ENTRY variable corresponds to your chosen namespace value. For this example, your `.env` file should look like this:

```.env
NAMESPACE=StampyConsole
ENTRY=src/console/
```
If the location of the controller does not suit you, you can always change it, for example :
```.env
NAMESPACE=App
ENTRY=src/
```

> [!WARNING]
> Because Stampy verifies every class and method in the namespace it uses, it's important to only use controllers that are compatible with Stampy. 
> in this namespace to avoid unnecessary verification during plugin execution.

You also have this stampy parameter created automatically in your installation.
```json
  "stampy": {
      "rebuild_after_install_or_update": false
  }
```
The following setting will rebuild your Stampy plugin every time you run `composer install` or `composer update`. Since this behavior can be annoying, it's disabled by default.

> [!WARNING]
> If this setting does not exist in your `composer.json` file the behavior will be considered true

### stampy's execution

## In a local context

After the local installation, a bash executable named `stampy` will be created. You can then run it using the following command:

```bash
  ./stampy
```
If you want to enable a specific command, enter it below like so :

```bash
  ./stampy bin
```
This will activate the bin command of your controller if it exists if it does not then the default command will be called it will show you the commands available in your controller
## In a containerize context

Then, once your custom Stampy container has been created, type the following command in your terminal.

```bash
  composer execdockerStamy
```
This will open a docker shell in your terminal in this shell type
```bash
  stampy
```
You would probably see something like this displayed:
```md
================================================================================
Command:bin
Description:---describe your command there---
================================================================================
```

## Controller usage
Using the plugin is quite simple in the namespace you had assigned in your `.env` folder create a class using the following attributes.
```php
namespace StampyConsole;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdOut;
use Stampy\Model\Attributes\StdIn;
use \Dialoguer;
use \NamespaceHandler;
use \Indicatif;
use function Stampy\padding;

class BinController extends AbstractPrompsController
{
	#[
		Command('bin'),/* This attribute allows you to activate
    the command, for example (./stampy bin) */
		Option([
			"-option1" => new Option_CLI(true,"---describe your option there---"),
      /* Each option is linked to a parameter, so "option1" corresponds to the parameter.
      $option1 option1 is either:
      - null I didn't write the option (./stampy bin)
      - bool(true) I wrote the option but didn't pass any input 
      (./stampy bin -option1)
      - string I wrote the option and passed an input 
      (./stampy bin -option1 abc) Here, the value of $option1 = abc
      Since Option_CLI has true as the first parameter, this means that this option accepts input.
      */
			"-option2" => new Option_CLI(false,"---describe your option there---"),
      /*
      So for the case of option 2 given that I have indicated by false in the Option_CLI class 
      that I do not want input for this option $option2 can only be bool or null and therefore 
      does not accept inputs an error is even sent if you all have an input for this file
      */
		]),
    StdOut("output-file.txt"),
    /* Corresponds to the assignment ">" to the call of a php file for example if you want 
    to redistribute a data stream in a particular file in this case all your echo will be redistributed
    inside and visible output will be redistributed inside the file in question if a file does 
    not exist it will be created automatically */
		Description('---describe your command there---'),
		StdErr("error.log"),
    /* Corresponds to the assignment "2>" when calling a php file, for example,
    if instead of outputting an error on a terminal you output it to this StdErr file,
    if a file does not exist it will be created automatically. */
		StdIn("input.json"),
    /* Corresponds to the assignment "<" to the call of a php file for example you wish to use 
    from the STDIN stream to give another file for your script if there is no file a bash error 
    will be raised telling you that you must first have a file at the origin to be able to use it correctly */
	]
	public function bin(
		null|bool|string $option1,
		null|bool $option2,
	){
		/* --- example of use ---- */
    stream_get_contents(STDIN); // return the content of the input.json file;
		echo "hello"; // will write in the file output.txt in the root of your project
		fwrite(STDERR,"log error"); // will write in the file error.log

    $this->color("hello world!","green") // green text = echo green text 
    // so if you echo the output could be redirect using stdout or err
    $this->color("hello world!","bggreen") // green background
    $this->color("hello world!","green","bold") // green text bold 
    $this->color("hello world!","green","bold","underline") // green text bold and underline
    $this->colorOut("Hello from output","green")
    //directly output a green text in your terminal (/dev/tty) even if there is a redirection 
    $this->colorErr("Hello from output","green")
    //directly output a green text in your terminal (/dev/tty) even if there is a redirection  
    //you need to flush your result to see it on your terminal 
    $this->TTY->getStdErrTTY()->flush();

    var_dump($option1,$option2) 
    /*
      |  input  (in docker shell)     |  input  (in local terminal)   |  param      |  value  | 
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin                   |  ./stampy bin                 |  $option1   |  null   |
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin                   |  ./stampy bin                 |  $option2   |  null   |
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin -option1          |  ./stampy bin -option1        |  $option1   |  true   |
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin -option2          |  ./stampy bin -option2        |  $option2   |  true   |
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin -option1 abc      |  ./stampy bin -option1 abc    |  $option1   |  "abc"  |
      |-------------------------------|-------------------------------|-------------|---------|
      |  stampy bin -option2 abc      |  ./stampy bin -option2 abc    |  $option1   |  ERROR  |
    */
    getenv("ENTRY") // global env variable "ENTRY" which is found in the .env
    /* --- the Foreign function interface (FFI)---- */
    // There are 3 external classes exported from the binary that can be used in your project
    /* 
      First, Dialoguer from the cargo library https://docs.rs/dialoguer/latest/dialoguer/,
      which has been adapted for this specific use.
      More information on its use can be found in the vendor/stampy/php-cli/app/FFI/Dialoguer.php folder.
    */
    $progress = $this->newProgressBar(100);
		for ($i=0;$i<99;$i++){
			$progress->increment();
			sleep(1);
		}
		$progress->finishAndClear();
    Dialoguer::input("Hi how are you ?",true) // input 
    /* 
      - Cargo library code: https://docs.rs/indice/latest/indice/, which has been adapted for this specific use.
      More information on its use can be found in the vendor/stampy/php-cli/app/FFI/Indecatif folder.
    */
    /* the class Indicatif is alrealdy implement in the abstract class 
    Stampy\Model\Abstract\AbstractPrompsController.php si you can you Indicatif like this */
    $progress = $this->newProgressBar(100);
		for ($i=0;$i<99;$i++){
			$progress->increment();
			sleep(1);
		}
		$progress->finishAndClear();
    // will display a clean loading bar with 100 long length
    // but you can use Indicatif with the same result as above
    $progress = new Indicatif(100);

    //this ffi will print you a padded input directly in your terminal /dev/tty/ 
    padding("hello, world!");

    /* 
        Namespace handler that allows you to return a well-formatted namespace array as well 
        as add or subtract a level.
        More information on its use can be found in the vendor/stampy/php-cli/app/FFI/NamespaceHandler.php folder.
    */
    $namespace = new NamespaceHandler(dirname(__DIR__,2)."scr/","App");
	}
}
```   
## ConsoleTTY

ConsoleTTY is a special mount class that help you reach directly the terminal tty even in case of redirection;

```php
namespace StampyConsole;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdOut;
use Stampy\Model\Attributes\StdIn;
use \Dialoguer;
use \NamespaceHandler;
use \Indicatif;
use function Stampy\padding;

class BinController extends AbstractPrompsController
{
	#[Command('bin')]
	public function bin(
		null|bool|string $option1,
		null|bool $option2,
	){
		/* --- for exemple -- */
    //the method getStdOutTTY will right directly in your /dev/tty 
    //this will right in your terminal no matter any redirection (">","2>","<")
    // so You can use the attribute StdErr|StdOut without any redirection 
		$this->TTY
    ->getStdOutTTY()
    ->write("hello");
    // the method getStdErrTTY will right directly in your /dev/tty but not right it down 
    $err = $this->TTY->getStdErrTTY();
		$err->write("abc");
		$err->write("efg");
    // when you flush it you will flush all your stderr input and then exit the code
    $err->flush();
	}
}
```   