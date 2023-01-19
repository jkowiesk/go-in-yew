# Serwer RustGo

Serwer korzysta z protokołu Websockets do utrzymywania połączeń z dwoma klientami gry równocześnie na różnych wątkach. Jego zadaniem jest synchronizacja stanu planszy pomiędzy graczami. Wszystkie wymieniane wiadomości są w formacie `json`.


## Przykładowa sekwencja działań i wiadomości wymienianych pomiędzy serwerem a klientami

1. Serwer zostaje uruchomiony.
2. Pierwszy klient wysyła do serwera wiadomość typu `join_game`.

```
{
    "message_type": "join_game"
}
```

3. Serwer odpowiada wiadomością tego samego typu, ale z dodatkowymi polami

```
{
    "message_type": "join_game",
    "status": "success",
    "player": "first"
}
```

4. Na podstawie wartości dla klucza `player` klient dowiaduje się że jest pierwszym graczem, dlatego wysyła do serwera wiadomość typu `initialize_board`.

```
{
    "message_type": "initialize_board",
    "board_size": 9
}
```

5. Serwer odpowiada wiadomością tego samego typu.

```
{
    "message_type": "initialize_board",
    "status": "success"
}
```

6. Do gry postanawia dołączyć drugi gracz, dlatego drugi klient wysyła wiadomość typu `join_game`

```
{
    "message_type": "join_game"
}
```

7. Uzyskuje on od serwera odpowiedź

```
{
    "message_type": "join_game",
    "status": "success",
    "player": "second"
}
```

8. Serwer rozpoznaje, że dwóch graczy dołączyło do rozgrywki, więc wysyła im obu wiadomość:

Do gracza pierwszego:
```
{
    "message_type": "board_state",
    "board": [0, 0, 0, 0, 0, 0, 0, 0, 0],
    "your_turn": true
}
```

Do gracza drugiego:
```
{
    "message_type": "board_state",
    "board": [0, 0, 0, 0, 0, 0, 0, 0, 0],
    "your_turn": false
}
```

9. Gracze na podstawie wartości dla `your_turn` przesyłają swoje wiadomości typu `board_state`

Zaczyna więc gracz pierwszy, wysłając np
```
{
    "message_type": "board_state",
    "board": [1, 0, 0, 0, 0, 0, 0, 0, 0]
}
```

Obydwaj gracze otrzymują wtedy wiadomość `board_state` ze zaktualizowaną tablicą oraz infrmacją o ruchu w `your_turn`

Do gracza pierwszego:
```
{
    "message_type": "board_state",
    "board": [1, 0, 0, 0, 0, 0, 0, 0, 0],
    "your_turn": false
}
```

Do gracza drugiego:
```
{
    "message_type": "board_state",
    "board": [1, 0, 0, 0, 0, 0, 0, 0, 0],
    "your_turn": true
}
```


## Możliwe formaty wiadomości

Zarówno serwer jak i klient rozpoznają typ wiadomości przesyłanej przez wartość dla klucza `message_type`.  
Dozwolone typy wiadomości klienta i odpowiedzi serwera:

### join_game

```
{
    "message_type": "join_game"
}
```

Klient wysyła tę wiadomość do serwera kiedy chce dołączyć do gry. Możliwe odpowiedzi serwera to:

```
{
    "message_type": "join_game",
    "status": "success",
    "player": "first"
}
```

Kiedy klient pierwszy dołaczy do gry, dostaje od serwera wiadomość którym jest graczem. Następną wysłaną przez niego wiadomością powinno być `initialize_board`, opisane w następnej sekcji.

```
{
    "message_type": "join_game",
    "status": "success",
    "player": "second"
}
```

Kiedy klient dołączy do gry jako drugi - powinien wtedy oczekiwać na wiadomość o stanie gry.


### initialize_board

```
{
    "message_type": "initialize_board",
    "board_size": 9
}
```

Kiedy klient po wysłaniu `join_game` uzyska wiadomość że jest pierwszym graczem, jest zobowiązany wybrać rozmiar planszy do gry. Możliwe jest otrzymanie od serwera jednej z poniższych wiadomości:

```
{
    "message_type": "initialize_board",
    "status": "success"
}
```

Kiedy wysłano poprawną wiadomość do serwera, i serwerowi udało się zainicjalizować planszę.

```
{
    "message_type": "initialize_board",
    "status": "error"
}
```

Kiedy nie udało się zainicjalizować planszy ze względu na zły format danych wiadomości.
