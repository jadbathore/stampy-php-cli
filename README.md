# Stampy/php-cli

Est un plugin composer permettant de facilité l'utilisation de l'invité de commande en php dans son projet de manière propre et organisé

## installation

> [!TIP]
> Composer va sûrment vous demander si vous souhaiter autorise durant l'installation. par ce message 

```bash
Package operations: 1 install, 0 updates, 0 removals
stampy/php-cli contains a Composer plugin which is currently not in your allow-plugins config. See https://getcomposer.org/allow-plugins
Do you trust "stampy/php-cli" to execute code and wish to enable it now? (writes "allow-plugins" to composer.json) [y,n,d,?] 
```
Repondez "y" a cette question.

> [!IMPORTANT]
> Stampy/php-cli est un plugin Composer il est donc important que vous acceptez dans votre dossier `composer.json` ce plugin de la manière suivante si ce n'est pas déja fait (composeur vous fomattera automatique ce paramètre si vous avez repondu "y" sinon dans composer ecrivez);

``` json
"config": {
        "allow-plugins": {
            "stampy/php-cli": true
        }
    },
```

Pour demarré l'installation de ce projet par le biais de composer faite la commande :
```bash
composer require stampy/php-cli
```
Par la suite après installation le plugin va verifier si une version pré-compilé de la librairie stampy est disponible pour votre achitecture.

### Si une version pré-compiler **existe** pour votre achitecture 

Alors vous veriez surment ce message 

```md
Generating autoload files
? the stampy extension add already a pre-compile binairy you can  ?  ›
  continue as such
  use docker
```
Cela veux dire que la librairie pré-compilé est bien compatible avec votre achitecture vous pouvez donc désormé choisir entre continue comme ça ou utilisé docker **un fichier bash executable est egalement ajouté a l'installation donc pas d'inquetude si vous souhaiter utilisé la librairie pré-compilé maintenant et par la suite contenurisé l'invite de commande**

Par la suite si vous utlisé plus d'un namespace dans votre projet stampy va vous demander dans quel namespace vous souhaiter mettre le controller qui vous permettra d'utilisé le plugin.

il créera également un dossier `.env` à la racine de votre projet composer grâce auquelle vous poviez utilsé des variables d'environement (uniquement pour la partie utilsant le plugin dans votre projet). 

> [!CAUTION]
> Ne **surtout** pas enlevè les variables namespace et entry dans votre dossier .env il sont essensiel au bon fonctionement du plugin vous *pouvez modifier ces variables si vous souhaiter activer le plugin depuis un autre endroit*.
### Si il **n'existe pas** de vesion pre-compilé pour votre achitecture

Alors vous veriez surment ce message
```bash 
the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself
using cargo or use docker.If you using cargo make sure you got cargo install (https://rust-lang.org/tools/install/).
If you using docker make sure you docker daemon running [cargo|docker] ?
```
Pas de problème grâce à cargo un compilation de la libraire stampy s'effectura et crera un executable personalisé pour votre achitecture (il faut prendre en considération que du faite que la librairie utilse diverse extension cargo elle n'est peux etre pas adapté a tout type d'achitecture si c'est malheuresment votre cas je vous conseille d'utilsé la version conteneurisé du plugin).

### Si vous souhaiter directement utilsé docker 

En suivant le process d'installation vous arrivé au meme resultat : le terminal vous demande de dans quel namespace vous souhaiter utilsé stampy puis vous crée dans votre app un controller / un .env mais également un fichier composer.stampy.json du faite que stampy en utilsant docker a un composer.json totalement séparé du composer de votre application se qui vous permet de séparé totalement ce que votre terminal peux faire et ce que votre application peux faire.



