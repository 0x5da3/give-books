```mermaid
erDiagram
    OGP ||--|{ BID : places
    OGP {
        string title PK
        string desc
        string url
        string image
    }
    
    BID {
        string title PK,FK
        int check
    }
```