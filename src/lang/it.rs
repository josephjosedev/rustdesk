lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stato"),
        ("Your Desktop", "Il tuo desktop"),
        ("desk_tip", "Puoi accedere al tuo desktop usando l'ID e la password riportati qui."),
        ("Password", "Password"),
        ("Ready", "Pronto"),
        ("Established", "Stabilito"),
        ("connecting_status", "Connessione alla rete RustDesk in corso..."),
        ("Enable Service", "Abilita servizio"),
        ("Start Service", "Avvia servizio"),
        ("Service is running", "Il servizio è in esecuzione"),
        ("Service is not running", "Il servizio non è in esecuzione"),
        ("not_ready_status", "Non pronto. Verifica la tua connessione"),
        ("Control Remote Desktop", "Controlla una scrivania remota"),
        ("Transfer File", "Trasferisci file"),
        ("Connect", "Connetti"),
        ("Recent Sessions", "Sessioni recenti"),
        ("Address Book", "Rubrica"),
        ("Confirmation", "Conferma"),
        ("TCP Tunneling", "Tunnel TCP"),
        ("Remove", "Rimuovi"),
        ("Refresh random password", "Nuova password casuale"),
        ("Set your own password", "Imposta la tua password"),
        ("Enable Keyboard/Mouse", "Abilita tastiera/mouse"),
        ("Enable Clipboard", "Abilita appunti"),
        ("Enable File Transfer", "Abilita trasferimento file"),
        ("Enable TCP Tunneling", "Abilita tunnel TCP"),
        ("IP Whitelisting", "IP autorizzati"),
        ("ID/Relay Server", "Server ID/Relay"),
        ("Stop service", "Arresta servizio"),
        ("Change ID", "Cambia ID"),
        ("Website", "Sito web"),
        ("About", "Informazioni"),
        ("Mute", "Silenzia"),
        ("Audio Input", "Input audio"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "ID server"),
        ("Relay Server", "Server relay"),
        ("API Server", "Server API"),
        ("invalid_http", "deve iniziare con http:// o https://"),
        ("Invalid IP", "Indirizzo IP non valido"),
        ("id_change_tip", "Puoi usare solo i caratteri a-z, A-Z, 0-9 e _ (underscore). Il primo carattere deve essere a-z o A-Z. La lunghezza deve essere fra 6 e 16 caratteri."),
        ("Invalid format", "Formato non valido"),
        ("server_not_support", "Non ancora supportato dal server"),
        ("Not available", "Non disponibile"),
        ("Too frequent", "Troppo frequente"),
        ("Cancel", "Annulla"),
        ("Skip", "Ignora"),
        ("Close", "Chiudi"),
        ("Retry", "Riprova"),
        ("OK", "OK"),
        ("Password Required", "Password richiesta"),
        ("Please enter your password", "Inserisci la tua password"),
        ("Remember password", "Ricorda password"),
        ("Wrong Password", "Password errata"),
        ("Do you want to enter again?", "Vuoi riprovare?"),
        ("Connection Error", "Errore di connessione"),
        ("Error", "Errore"),
        ("Reset by the peer", "Reimpostata dal peer"),
        ("Connecting...", "Connessione..."),
        ("Connection in progress. Please wait.", "Connessione in corso. Attendi."),
        ("Please try 1 minute later", "Per favore riprova fra 1 minuto"),
        ("Login Error", "Errore di login"),
        ("Successful", "Successo"),
        ("Connected, waiting for image...", "Connesso, in attesa dell'immagine..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificato"),
        ("Size", "Dimensione"),
        ("Show Hidden Files", "Mostra file nascosti"),
        ("Receive", "Ricevi"),
        ("Send", "Invia"),
        ("Refresh File", "Aggiorna file"),
        ("Local", "Locale"),
        ("Remote", "Remote"),
        ("Remote Computer", "Computer remoto"),
        ("Local Computer", "Computer locale"),
        ("Confirm Delete", "Conferma cancellazione"),
        ("Delete", "Eliminare"),
        ("Properties", "Proprietà"),
        ("Multi Select", "Selezione multipla"),
        ("Empty Directory", "Directory vuota"),
        ("Not an empty directory", "Non una directory vuota"),
        ("Are you sure you want to delete this file?", "Vuoi davvero eliminare questo file?"),
        ("Are you sure you want to delete this empty directory?", "Sei sicuro di voler eliminare questa directory vuota?"),
        ("Are you sure you want to delete the file of this directory?", "Sei sicuro di voler eliminare il file di questa directory?"),
        ("Do this for all conflicts", "Ricorca questa scelta per tutti i conflitti"),
        ("This is irreversible!", "Questo è irreversibile!"),
        ("Deleting", "Cancellazione di"),
        ("files", "file"),
        ("Waiting", "In attesa"),
        ("Finished", "Terminato"),
        ("Speed", "Velocità"),
        ("Custom Image Quality", "Qualità immagine personalizzata"),
        ("Privacy mode", "Modalità privacy"),
        ("Block user input", "Blocca l'input dell'utente"),
        ("Unblock user input", "Sbloccare l'input dell'utente"),
        ("Adjust Window", "Adatta la finestra"),
        ("Original", "Originale"),
        ("Shrink", "Restringi"),
        ("Stretch", "Allarga"),
        ("Good image quality", "Buona qualità immagine"),
        ("Balanced", "Bilanciato"),
        ("Optimize reaction time", "Ottimizza il tempo di reazione"),
        ("Custom", "Personalizzato"),
        ("Show remote cursor", "Mostra il cursore remoto"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Disabilita appunti"),
        ("Lock after session end", "Blocca al termine della sessione"),
        ("Insert", "Inserisci"),
        ("Insert Lock", "Blocco inserimento"),
        ("Refresh", "Aggiorna"),
        ("ID does not exist", "L'ID non esiste"),
        ("Failed to connect to rendezvous server", "Errore di connessione al server rendezvous"),
        ("Please try later", "Riprova più tardi"),
        ("Remote desktop is offline", "Il desktop remoto è offline"),
        ("Key mismatch", "La chiave non corrisponde"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "Errore di connessione al server relay"),
        ("Failed to connect via rendezvous server", "Errore di connessione tramite il server rendezvous"),
        ("Failed to connect via relay server", "Errore di connessione tramite il server relay"),
        ("Failed to make direct connection to remote desktop", "Impossibile connettersi direttamente al desktop remoto"),
        ("Set Password", "Imposta password"),
        ("OS Password", "Password del sistema operativo"),
        ("install_tip", "A causa del Controllo Account Utente, RustDesk potrebbe non funzionare correttamente come desktop remoto. Per evitare questo problema, fai click sul tasto qui sotto per installare RustDesk a livello di sistema."),
        ("Click to upgrade", "Fai click per aggiornare"),
        ("Click to download", "Cliquez per scaricare"),
        ("Click to update", "Fare clic per aggiornare"),
        ("Configure", "Configura"),
        ("config_acc", "Per controllare il tuo desktop dall'esterno, devi fornire a RustDesk il permesso \"Accessibilità\"."),
        ("config_screen", "Per controllare il tuo desktop dall'esterno, devi fornire a RustDesk il permesso \"Registrazione schermo\"."),
        ("Installing ...", "Installazione ..."),
        ("Install", "Installa"),
        ("Installation", "Installazione"),
        ("Installation Path", "Percorso di installazione"),
        ("Create start menu shortcuts", "Crea i collegamenti nel menu di avvio"),
        ("Create desktop icon", "Crea un'icona sul desktop"),
        ("agreement_tip", "Avviando l'installazione, accetti i termini del contratto di licenza."),
        ("Accept and Install", "Accetta e installa"),
        ("End-user license agreement", "Contratto di licenza con l'utente finale"),
        ("Generating ...", "Generazione ..."),
        ("Your installation is lower version.", "La tua installazione non è aggiornata."),
        ("not_close_tcp_tip", "Non chiudere questa finestra mentre stai usando il tunnel"),
        ("Listening ...", "In ascolto ..."),
        ("Remote Host", "Host remoto"),
        ("Remote Port", "Porta remota"),
        ("Action", "Azione"),
        ("Add", "Aggiungi"),
        ("Local Port", "Porta locale"),
        ("setup_server_tip", "Per una connessione più veloce, configura un tuo server"),
        ("Too short, at least 6 characters.", "Troppo breve, almeno 6 caratteri"),
        ("The confirmation is not identical.", "La conferma non corrisponde"),
        ("Permissions", "Permessi"),
        ("Accept", "Accetta"),
        ("Dismiss", "Rifiuta"),
        ("Disconnect", "Disconnetti"),
        ("Allow using keyboard and mouse", "Consenti l'uso di tastiera e mouse"),
        ("Allow using clipboard", "Consenti l'uso degli appunti"),
        ("Allow hearing sound", "Consenti la riproduzione dell'audio"),
        ("Allow file copy and paste", "Consenti copia e incolla di file"),
        ("Connected", "Connesso"),
        ("Direct and encrypted connection", "Connessione diretta e cifrata"),
        ("Relayed and encrypted connection", "Connessione tramite relay e cifrata"),
        ("Direct and unencrypted connection", "Connessione diretta e non cifrata"),
        ("Relayed and unencrypted connection", "Connessione tramite relay e non cifrata"),
        ("Enter Remote ID", "Inserisci l'ID remoto"),
        ("Enter your password", "Inserisci la tua password"),
        ("Logging in...", "Autenticazione..."),
        ("Enable RDP session sharing", "Abilita la condivisione della sessione RDP"),
        ("Auto Login", "Login automatico"),
        ("Enable Direct IP Access", "Abilita l'accesso diretto tramite IP"),
        ("Rename", "Rinomina"),
        ("Space", "Spazio"),
        ("Create Desktop Shortcut", "Crea collegamento sul desktop"),
        ("Change Path", "Cambia percorso"),
        ("Create Folder", "Crea cartella"),
        ("Please enter the folder name", "Inserisci il nome della cartella"),
        ("Fix it", "Risolvi"),
        ("Warning", "Avviso"),
        ("Login screen using Wayland is not supported", "La schermata di login non è supportata utilizzando Wayland"),
        ("Reboot required", "Riavvio necessario"),
        ("Unsupported display server ", "Display server non supportato"),
        ("x11 expected", "x11 necessario"),
        ("Port", "Porta"),
        ("Settings", "Impostazioni"),
        ("Username", " Nome utente"),
        ("Invalid port", "Porta non valida"),
        ("Closed manually by the peer", "Chiuso manualmente dal peer"),
        ("Enable remote configuration modification", "Abilita la modifica remota della configurazione"),
        ("Run without install", "Avvia senza installare"),
        ("Always connected via relay", "Connesso sempre tramite relay"),
        ("Always connect via relay", "Connetti sempre tramite relay"),
        ("whitelist_tip", "Solo gli indirizzi IP autorizzati possono connettersi a questo desktop"),
        ("Login", "Accedi"),
        ("Logout", "Esci"),
        ("Tags", "Tag"),
        ("Search ID", "Cerca ID"),
        ("Current Wayland display server is not supported", "Questo display server Wayland non è supportato"),
        ("whitelist_sep", "Separati da virgola, punto e virgola, spazio o a capo"),
        ("Add ID", "Aggiungi ID"),
        ("Add Tag", "Aggiungi tag"),
        ("Unselect all tags", "Deseleziona tutti i tag"),
        ("Network error", "Errore di rete"),
        ("Username missed", "Nome utente dimenticato"),
        ("Password missed", "Password dimenticata"),
        ("Wrong credentials", "Credenziali errate"),
        ("Edit Tag", "Modifica tag"),
        ("Unremember Password", "Dimentica password"),
        ("Favorites", "Preferiti"),
        ("Add to Favorites", "Aggiungi ai preferiti"),
        ("Remove from Favorites", "Rimuovi dai preferiti"),
        ("Empty", "Vuoto"),
        ("Invalid folder name", "Nome della cartella non valido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nome host"),
        ("Discovered", "Rilevati"),
        ("install_daemon_tip", "Per avviarsi all'accensione, è necessario installare il servizio di sistema."),
        ("Remote ID", "ID remoto"),
        ("Paste", "Impasto"),
        ("Paste here?", "Incolla qui?"),
        ("Are you sure to close the connection?", "Sei sicuro di voler chiudere la connessione?"),
        ("Download new version", "Scarica nuova versione"),
        ("Touch mode", "Modalità tocco"),
        ("Mouse mode", "Modalità mouse"),
        ("One-Finger Tap", "Tocca con un dito"),
        ("Left Mouse", "Mouse sinistro"),
        ("One-Long Tap", "Tocco lungo con un dito"),
        ("Two-Finger Tap", "Tocca con due dita"),
        ("Right Mouse", "Mouse destro"),
        ("One-Finger Move", "Movimento con un dito"),
        ("Double Tap & Move", "Tocca due volte e sposta"),
        ("Mouse Drag", "Trascina il mouse"),
        ("Three-Finger vertically", "Tre dita in verticale"),
        ("Mouse Wheel", "Rotellina del mouse"),
        ("Two-Finger Move", "Movimento con due dita"),
        ("Canvas Move", "Sposta tela"),
        ("Pinch to Zoom", "Pizzica per zoomare"),
        ("Canvas Zoom", "Zoom tela"),
        ("Reset canvas", "Ripristina tela"),
        ("No permission of file transfer", "Nessun permesso di trasferimento di file"),
        ("Note", "Nota"),
        ("Connection", "Connessione"),
        ("Share Screen", "Condividi schermo"),
        ("CLOSE", "CHIUDERE"),
        ("OPEN", "APRIRE"),
        ("Chat", "Chat"),
        ("Total", "Totale"),
        ("items", "Oggetti"),
        ("Selected", "Selezionato"),
        ("Screen Capture", "Cattura schermo"),
        ("Input Control", "Controllo di input"),
        ("Audio Capture", "Acquisizione audio"),
        ("File Connection", "Connessione file"),
        ("Screen Connection", "Connessione schermo"),
        ("Do you accept?", "Accetti?"),
        ("Open System Setting", "Apri impostazioni di sistema"),
        ("How to get Android input permission?", "Come ottenere l'autorizzazione di input su Android?"),
        ("android_input_permission_tip1", "Affinché un dispositivo remoto possa controllare il tuo dispositivo Android tramite mouse o tocco, devi consentire a RustDesk di utilizzare il servizio \"Accessibilità\"."),
        ("android_input_permission_tip2", "Vai alla pagina delle impostazioni di sistema che si aprirà di seguito, trova e accedi a [Servizi installati], attiva il servizio [RustDesk Input]."),
        ("android_new_connection_tip", "È stata ricevuta una nuova richiesta di controllo per il dispositivo corrente."),
        ("android_service_will_start_tip", "L'attivazione di Cattura schermo avvierà automaticamente il servizio, consentendo ad altri dispositivi di richiedere una connessione da questo dispositivo."),
        ("android_stop_service_tip", "La chiusura del servizio chiuderà automaticamente tutte le connessioni stabilite."),
        ("android_version_audio_tip", "L'attuale versione di Android non supporta l'acquisizione audio, esegui l'upgrade ad Android 10 o versioni successive."),
        ("android_start_service_tip", "Toccare [Avvia servizio] o APRI l'autorizzazione [Cattura schermo] per avviare il servizio di condivisione dello schermo."),
        ("Account", "Account"),
        ("Overwrite", "Sovrascrivi"),
        ("This file exists, skip or overwrite this file?", "Questo file esiste, saltare o sovrascrivere questo file?"),
        ("Quit", "Esci"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Aiuto"),
        ("Failed", "Fallito"),
        ("Succeeded", "Successo"),
        ("Someone turns on privacy mode, exit", "Qualcuno attiva la modalità privacy, esci"),
        ("Unsupported", "Non supportato"),
        ("Peer denied", "Pari negato"),
        ("Please install plugins", "Si prega di installare i plugin"),
        ("Peer exit", "Uscita tra pari"),
        ("Failed to turn off", "Impossibile spegnere"),
        ("Turned off", "Spegni"),
        ("In privacy mode", "In modalità privacy"),
        ("Out privacy mode", "Fuori modalità privacy"),
        ("Language", "Linguaggio"),
    ].iter().cloned().collect();
}
