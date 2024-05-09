-- Your SQL goes here
INSERT INTO "users" ("id", "created_at", "first_name", "last_name", "user_role") VALUES
('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3',	'2024-05-09 10:35:25.311141+00',	'romain',	'allouch',	'HOST'),
('usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f',	'2024-05-09 10:44:48.402852+00',	'luka',	'lazic',	'HOST'),
('usr-010fdf0c-fbf8-4064-844b-ab427334e8f1',	'2024-05-09 10:49:23.039292+00',	'timothee',	'gerberon',	'GUEST'),
('usr-39824616-0726-4c1f-b957-5c11f782783c',	'2024-05-09 11:14:51.093101+00',	'laurent',	'cerveau',	'GUEST');

INSERT INTO "offices" ("id", "created_at", "name", "address_text", "latitude", "longitude", "surface", "num_posts", "price_per_post", "parent_office_id", "owner_id") VALUES
('ofc-caad197b-fa5b-4174-913c-d145a80a8952',	'2024-05-09 10:38:37.779452+00',	'franklin ',	'1 rue du franklin , Paris 75008',	-68.11465134874877,	8.607017518204287,	110,	70,	300,	NULL,	'usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3'),
('ofc-dfc845a2-fc99-4df1-8058-b6c7f03743f2',	'2024-05-09 10:40:13.799557+00',	'maison rouge',	'2 rue de la rougeur, Paris 75010',	-43.171187757425855,	-105.40005454282482,	500,	180,	450,	NULL,	'usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3'),
('ofc-c05df406-5df5-4305-b76c-c04d15036814',	'2024-05-09 10:43:26.033538+00',	'lorette',	'21 rue de chateaudun, Paris 75009',	21.585819874547738,	-114.94346585509932,	80,	40,	300,	NULL,	'usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3'),
('ofc-a20f96ec-df7a-4b45-8ef4-72360909f019',	'2024-05-09 10:46:03.56994+00',	'nuages',	'23 rue du soleil, Paris 75001',	-44.000729492908285,	76.84215918141274,	450,	180,	500,	NULL,	'usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f'),
('ofc-b2d51f5c-0aea-41e3-a568-369598037530',	'2024-05-09 10:48:42.921065+00',	'opera',	'10 rue de la paix, Paris 75009',	-20.17197067014979,	-37.28504994092816,	180,	100,	390,	NULL,	'usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f');

INSERT INTO "host_relations" ("user_id", "office_id", "created_at") VALUES
('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3',	'ofc-caad197b-fa5b-4174-913c-d145a80a8952',	'2024-05-09 10:38:37.804099+00'),
('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3',	'ofc-dfc845a2-fc99-4df1-8058-b6c7f03743f2',	'2024-05-09 10:40:13.817654+00'),
('usr-8ea33211-d0ab-4d93-b8ce-bb9dec2f67f3',	'ofc-c05df406-5df5-4305-b76c-c04d15036814',	'2024-05-09 10:43:26.049594+00'),
('usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f',	'ofc-a20f96ec-df7a-4b45-8ef4-72360909f019',	'2024-05-09 10:46:03.596049+00'),
('usr-9d8166ca-3fea-43a8-9aa3-c4d7aa7d840f',	'ofc-b2d51f5c-0aea-41e3-a568-369598037530',	'2024-05-09 10:48:42.945842+00');