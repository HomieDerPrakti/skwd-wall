settings-library-watch-section-desc = Détecte les fichiers ajoutés hors de skwd-wall et garde la bibliothèque à jour.
settings-library-watch-fallback-label = Repli par sondage
settings-library-watch-fallback-desc = Vérifie uniquement les dossiers de la bibliothèque que la surveillance native des fichiers ne peut pas suivre. Activez ceci pour les montages réseau ou FUSE qui manquent des changements, puis redémarrez skwd-walld.
settings-library-watch-interval-label = Intervalle de sondage
settings-library-watch-interval-desc = Attend ce nombre de secondes entre deux vérifications bornées. Des valeurs plus basses trouvent les changements plus tôt mais lisent le système de fichiers plus souvent. Redémarrez skwd-walld après modification.
settings-library-watch-unknown-label = État de la surveillance indisponible
settings-library-watch-unknown-desc = Ce daemon ne signale pas l’état de surveillance de la bibliothèque. Mettez à jour ou redémarrez skwd-walld.
settings-library-watch-poll-failed-label = Le sondage ne peut pas lire un dossier de la bibliothèque
settings-library-watch-poll-failed-desc = Vérifiez que chaque dossier de bibliothèque configuré est monté et lisible. Le sondage réessaiera dans { $interval } secondes.
settings-library-watch-polling-label = Repli par sondage actif
settings-library-watch-polling-desc = La surveillance native a échoué pour { $count ->
    [one] { $count } dossier de la bibliothèque
   *[other] { $count } dossiers de la bibliothèque
    }. Jusqu’à { $budget } entrées sont vérifiées toutes les { $interval } secondes. Dernière convergence réussie : { $convergence }.
settings-library-watch-recovering-label = Surveillance native rétablie
settings-library-watch-recovering-desc = Le surveillant natif est de nouveau actif. Une analyse complète de transfert est encore en cours avant que la bibliothèque soit déclarée à jour.
settings-library-watch-unavailable-label = Surveillance de la bibliothèque indisponible
settings-library-watch-unavailable-desc = La surveillance native des fichiers a échoué et le repli par sondage est désactivé. Activez Repli par sondage, puis redémarrez skwd-walld.
settings-library-watch-recovered-label = Surveillance native restaurée
settings-library-watch-recovered-desc = Le surveillant natif et son analyse de transfert sont à jour. Dernière convergence réussie : { $convergence }.
settings-library-watch-native-label = Surveillance native des fichiers
settings-library-watch-native-desc = Les événements du système de fichiers sont actifs pour chaque dossier de la bibliothèque. Le sondage est inactif.
settings-library-watch-convergence-never = Pas encore terminée
settings-library-watch-convergence-seconds = { $value ->
    [one] il y a { $value } seconde
   *[other] il y a { $value } secondes
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] il y a { $value } minute
   *[other] il y a { $value } minutes
    }
settings-library-watch-convergence-hours = { $value ->
    [one] il y a { $value } heure
   *[other] il y a { $value } heures
    }
