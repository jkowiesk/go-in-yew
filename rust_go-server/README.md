# Serwer RustGo

Serwer korzysta z protokołu Websockets do utrzymywania połączeń z dwoma klientami gry równocześnie na różnych wątkach. Jego zadaniem jest synchronizacja stanu planszy pomiędzy graczami.

## Format wiadomości klienta

Istnieją dwa rodzaje wiadmomości które klient może wysłać do serwera:

1. Inicjalizacja planszy
Pierwszy klient dołączający do rozgrywki powininen pierwszą wiadomością do serwera ustalić rozmiar tablicy reprezentującej planszę. Przykładowo, dla planszy o rozmiarze zdefiniowanym w Go jako 9x9 powinna być to liczba 100:

```
{
    "board_size": 100
}
```

2. Przesyłanie uaktualnionego stanu planszy
Po wykonaniu swojego ruchu klient powinine odesłać serwerowi informację o tym, w jakim stanie aktualnie znajduje się plansza

```
{
    "board_size": [0, 2, 3, 0, 1, 1]
}
```

## Format wiadomości servera

Serwer odpowiada na **poprawne** zapytania klienta zawsze w ten sposób - zwracając aktualny stan planszy oraz informację, czy dany klient powinien się ruszyć w danej turze, na przykład:

```
{
    "your_turn": false,
    "board": [0, 1, 1, 1, 0, 0, 1, 0, 1] 
}
```

Serwer zwraca też odpowiednie komunikaty w przypadku przesłania niepoprowanych danych.
