-- This file should undo anything in `up.sql`

DELETE FROM "host_relations" WHERE 
    ("user_id", "office_id") IN (
        ('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3', 'ofc-caad197b-fa5b-4174-913c-d145a80a8952'),
        ('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3', 'ofc-dfc845a2-fc99-4df1-8058-b6c7f03743f2'),
        ('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3', 'ofc-c05df406-5df5-4305-b76c-c04d15036814'),
        ('usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f', 'ofc-a20f96ec-df7a-4b45-8ef4-72360909f019'),
        ('usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f', 'ofc-b2d51f5c-0aea-41e3-a568-369598037530')
    );

DELETE FROM "offices" WHERE "id" IN (
    'ofc-caad197b-fa5b-4174-913c-d145a80a8952',
    'ofc-dfc845a2-fc99-4df1-8058-b6c7f03743f2',
    'ofc-c05df406-5df5-4305-b76c-c04d15036814',
    'ofc-a20f96ec-df7a-4b45-8ef4-72360909f019',
    'ofc-b2d51f5c-0aea-41e3-a568-369598037530'
);

DELETE FROM "users" WHERE "id" IN (
    'usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3',
    'usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f',
    'usr-010fdf0c-fbf8-4064-844b-ab427334e8f1',
    'usr-39824616-0726-4c1f-b957-5c11f782783c');




