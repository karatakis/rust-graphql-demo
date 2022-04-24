Schema {
    tables: [
        TableDef {
            name: "albums",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "artists",
                    from: "ArtistId",
                    to: "ArtistId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "AlbumId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Title",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "ArtistId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "artists",
            foreign_keys: [],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "ArtistId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Name",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "customers",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "employees",
                    from: "SupportRepId",
                    to: "EmployeeId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "CustomerId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "FirstName",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "LastName",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 3,
                    name: "Company",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 4,
                    name: "Address",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 5,
                    name: "City",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 6,
                    name: "State",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 7,
                    name: "Country",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 8,
                    name: "PostalCode",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 9,
                    name: "Phone",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 10,
                    name: "Fax",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 11,
                    name: "Email",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 12,
                    name: "SupportRepId",
                    type: Integer,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "employees",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "employees",
                    from: "ReportsTo",
                    to: "EmployeeId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "EmployeeId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "LastName",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "FirstName",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 3,
                    name: "Title",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 4,
                    name: "ReportsTo",
                    type: Integer,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 5,
                    name: "BirthDate",
                    type: DateTime,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 6,
                    name: "HireDate",
                    type: DateTime,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 7,
                    name: "Address",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 8,
                    name: "City",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 9,
                    name: "State",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 10,
                    name: "Country",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 11,
                    name: "PostalCode",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 12,
                    name: "Phone",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 13,
                    name: "Fax",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 14,
                    name: "Email",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "genres",
            foreign_keys: [],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "GenreId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Name",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "invoices",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "customers",
                    from: "CustomerId",
                    to: "CustomerId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "InvoiceId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "CustomerId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "InvoiceDate",
                    type: DateTime,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 3,
                    name: "BillingAddress",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 4,
                    name: "BillingCity",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 5,
                    name: "BillingState",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 6,
                    name: "BillingCountry",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 7,
                    name: "BillingPostalCode",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 8,
                    name: "Total",
                    type: Numeric,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "invoice_items",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "tracks",
                    from: "TrackId",
                    to: "TrackId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
                ForeignKeysInfo {
                    id: 1,
                    seq: 0,
                    table: "invoices",
                    from: "InvoiceId",
                    to: "InvoiceId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "InvoiceLineId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "InvoiceId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "TrackId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 3,
                    name: "UnitPrice",
                    type: Numeric,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 4,
                    name: "Quantity",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "media_types",
            foreign_keys: [],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "MediaTypeId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Name",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "playlists",
            foreign_keys: [],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "PlaylistId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Name",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "playlist_track",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "tracks",
                    from: "TrackId",
                    to: "TrackId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
                ForeignKeysInfo {
                    id: 1,
                    seq: 0,
                    table: "playlists",
                    from: "PlaylistId",
                    to: "PlaylistId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "PlaylistId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "TrackId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
            ],
            auto_increment: false,
        },
        TableDef {
            name: "tracks",
            foreign_keys: [
                ForeignKeysInfo {
                    id: 0,
                    seq: 0,
                    table: "media_types",
                    from: "MediaTypeId",
                    to: "MediaTypeId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
                ForeignKeysInfo {
                    id: 1,
                    seq: 0,
                    table: "genres",
                    from: "GenreId",
                    to: "GenreId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
                ForeignKeysInfo {
                    id: 2,
                    seq: 0,
                    table: "albums",
                    from: "AlbumId",
                    to: "AlbumId",
                    on_update: NoAction,
                    on_delete: NoAction,
                    match: None,
                },
            ],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "TrackId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: true,
                },
                ColumnInfo {
                    cid: 1,
                    name: "Name",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "AlbumId",
                    type: Integer,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 3,
                    name: "MediaTypeId",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 4,
                    name: "GenreId",
                    type: Integer,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 5,
                    name: "Composer",
                    type: NvarChar {
                        length: 255,
                    },
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 6,
                    name: "Milliseconds",
                    type: Integer,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 7,
                    name: "Bytes",
                    type: Integer,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 8,
                    name: "UnitPrice",
                    type: Numeric,
                    not_null: true,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: true,
        },
        TableDef {
            name: "sqlite_stat1",
            foreign_keys: [],
            columns: [
                ColumnInfo {
                    cid: 0,
                    name: "tbl",
                    type: Blob,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 1,
                    name: "idx",
                    type: Blob,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
                ColumnInfo {
                    cid: 2,
                    name: "stat",
                    type: Blob,
                    not_null: false,
                    default_value: Unspecified,
                    primary_key: false,
                },
            ],
            auto_increment: false,
        },
    ],
}