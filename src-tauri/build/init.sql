DEFINE NAMESPACE Dispatcher;
USE NS Dispatcher;
DEFINE DATABASE DispatcherDB;
USE DB DispatcherDB;

define table role schemafull
          permissions
            for create, update NONE,
            for select where $auth.roles containsany [role:declarant, role:client, role:representative, role:inspector, role:operator],
            for delete NONE;
            create role:declarant; 
            create role:client;
            create role:representative;
            create role:inspector;
            create role:operator;

DEFINE TABLE user SCHEMAFULL
            PERMISSIONS 
                FOR select, update WHERE id = $auth.id, 
                FOR create, delete NONE;
DEFINE FIELD user ON user TYPE string;
DEFINE FIELD pass ON user TYPE string;
DEFINE FIELD role ON user TYPE record(role);
DEFINE FIELD additional_info ON user TYPE array;
DEFINE INDEX idx_user ON user COLUMNS user UNIQUE;

DEFINE SCOPE DispatcherScope
            SESSION 1h
            SIGNUP ( CREATE user SET user = $user, pass = crypto::argon2::generate($pass), role = $role, additional_info = $additional_info )
            SIGNIN ( SELECT * FROM user WHERE user = $user AND crypto::argon2::compare(pass, $pass) );


DEFINE TABLE declaration SCHEMAFULL
            PERMISSIONS 
                FOR select WHERE true, 
                FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:declarant, role:client, role:representative, role:inspector];

        DEFINE FIELD signed_by ON declaration TYPE record(representative, declarant);
        DEFINE FIELD inspected_by ON declaration TYPE record(inspector);
        DEFINE FIELD product_name ON declaration TYPE string;
        DEFINE FIELD product_code ON declaration TYPE string;
        DEFINE FIELD product_price ON declaration TYPE float
            ASSERT $value >= 0;
        DEFINE FIELD product_quantity ON declaration TYPE int
            ASSERT $value > 0;
        DEFINE FIELD product_weight ON declaration TYPE float
            ASSERT $value > 0;
        DEFINE FIELD product_description ON declaration TYPE string;
        DEFINE FIELD transport_type ON declaration TYPE string;
        DEFINE FIELD transport_name ON declaration TYPE string;
        DEFINE FIELD sender_name ON declaration TYPE string;
        DEFINE FIELD receiver_name ON declaration TYPE string;
        DEFINE FIELD destination ON declaration TYPE string;
        DEFINE FIELD departure ON declaration TYPE string;
        DEFINE FIELD state ON declaration TYPE string;
        DEFINE FIELD created_at ON declaration TYPE datetime;
        DEFINE FIELD updated_at ON declaration TYPE datetime;
        DEFINE INDEX idx_declaration ON declaration COLUMNS id UNIQUE;

DEFINE TABLE customs SCHEMAFULL
            PERMISSIONS
                FOR select WHERE true,
                FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:operator];

        DEFINE FIELD work_hours ON customs TYPE array;
        DEFINE FIELD location ON customs TYPE record(location);
        DEFINE FIELD competence ON customs TYPE string;
        DEFINE FIELD phone_number ON customs TYPE string;
        DEFINE FIELD email ON customs TYPE string;
        DEFINE FIELD declarations ON customs TYPE array;
        DEFINE FIELD declarations.* ON customs TYPE record(declaration);
        DEFINE FIELD inspectors ON customs TYPE array;
        DEFINE FIELD operators ON customs TYPE array;
        DEFINE FIELD customs_params ON customs TYPE object;

DEFINE TABLE inspector SCHEMAFULL
            PERMISSIONS
                FOR select WHERE true,
                FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:inspector];

        DEFINE FIELD name ON inspector TYPE string;
        DEFINE FIELD rank ON inspector TYPE string;
        DEFINE FIELD post ON inspector TYPE string;
        DEFINE FIELD declarations ON inspector TYPE array;
        DEFINE FIELD declarations.* ON inspector TYPE record(declaration);

DEFINE TABLE declarant SCHEMAFULL
            PERMISSIONS
                FOR select WHERE true,
                FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:declarant];
                
        DEFINE FIELD name ON declarant TYPE string;
        -- DEFINE FIELD location ON declarant TYPE record(location);
        -- DEFINE FIELD declarations ON declarant TYPE array;
        -- DEFINE FIELD declarations.* ON declarant TYPE record(declaration);

DEFINE TABLE representative SCHEMAFULL
            PERMISSIONS
                FOR select WHERE true,
                FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:representative];
                
        DEFINE FIELD name ON declarant TYPE string;
        DEFINE FIELD location ON declarant TYPE record(location);
        DEFINE FIELD declarations ON declarant TYPE array;
        DEFINE FIELD declarations.* ON declarant TYPE record(declaration);


