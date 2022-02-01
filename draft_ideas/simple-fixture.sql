--
-- PostgreSQL database dump
--

-- Dumped from database version 14.1 (Debian 14.1-1.pgdg110+1)
-- Dumped by pg_dump version 14.1 (Ubuntu 14.1-2.pgdg20.04+1)

-- Started on 2022-02-01 22:44:30 CET

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 217 (class 1259 OID 16884)
-- Name: grammar_tag; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.grammar_tag (
    id integer NOT NULL,
    name character varying(32) NOT NULL
);


ALTER TABLE public.grammar_tag OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16883)
-- Name: grammar_tag_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.grammar_tag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.grammar_tag_id_seq OWNER TO postgres;

--
-- TOC entry 3397 (class 0 OID 0)
-- Dependencies: 216
-- Name: grammar_tag_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.grammar_tag_id_seq OWNED BY public.grammar_tag.id;


--
-- TOC entry 211 (class 1259 OID 16849)
-- Name: non_terminal_symbol; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.non_terminal_symbol (
    id integer NOT NULL,
    name character varying(8) NOT NULL
);


ALTER TABLE public.non_terminal_symbol OWNER TO postgres;

--
-- TOC entry 210 (class 1259 OID 16848)
-- Name: non_terminal_symbol_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.non_terminal_symbol_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.non_terminal_symbol_id_seq OWNER TO postgres;

--
-- TOC entry 3398 (class 0 OID 0)
-- Dependencies: 210
-- Name: non_terminal_symbol_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.non_terminal_symbol_id_seq OWNED BY public.non_terminal_symbol.id;


--
-- TOC entry 213 (class 1259 OID 16858)
-- Name: production; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.production (
    id integer NOT NULL,
    non_terminal_symbol integer NOT NULL,
    production character varying(1024) NOT NULL,
    nts_amount integer GENERATED ALWAYS AS ((character_length((production)::text) - character_length(replace((production)::text, '{'::text, ''::text)))) STORED NOT NULL
);


ALTER TABLE public.production OWNER TO postgres;

--
-- TOC entry 212 (class 1259 OID 16857)
-- Name: production_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.production_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.production_id_seq OWNER TO postgres;

--
-- TOC entry 3399 (class 0 OID 0)
-- Dependencies: 212
-- Name: production_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.production_id_seq OWNED BY public.production.id;


--
-- TOC entry 219 (class 1259 OID 16893)
-- Name: semantic_tag; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.semantic_tag (
    id integer NOT NULL,
    name character varying(32) NOT NULL,
    sticky boolean DEFAULT true NOT NULL
);


ALTER TABLE public.semantic_tag OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 16892)
-- Name: semantic_tag_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.semantic_tag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.semantic_tag_id_seq OWNER TO postgres;

--
-- TOC entry 3400 (class 0 OID 0)
-- Dependencies: 218
-- Name: semantic_tag_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.semantic_tag_id_seq OWNED BY public.semantic_tag.id;


--
-- TOC entry 215 (class 1259 OID 16875)
-- Name: word; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.word (
    id integer NOT NULL,
    content character varying(64) NOT NULL,
    non_repeatable boolean DEFAULT true NOT NULL
);


ALTER TABLE public.word OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 16923)
-- Name: word_grammar; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.word_grammar (
    id integer NOT NULL,
    word integer NOT NULL,
    grammar_tag integer NOT NULL
);


ALTER TABLE public.word_grammar OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 16922)
-- Name: word_grammar_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.word_grammar_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.word_grammar_id_seq OWNER TO postgres;

--
-- TOC entry 3401 (class 0 OID 0)
-- Dependencies: 222
-- Name: word_grammar_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.word_grammar_id_seq OWNED BY public.word_grammar.id;


--
-- TOC entry 214 (class 1259 OID 16874)
-- Name: word_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.word_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.word_id_seq OWNER TO postgres;

--
-- TOC entry 3402 (class 0 OID 0)
-- Dependencies: 214
-- Name: word_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.word_id_seq OWNED BY public.word.id;


--
-- TOC entry 221 (class 1259 OID 16903)
-- Name: word_semantic; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.word_semantic (
    id integer NOT NULL,
    word integer NOT NULL,
    semantic_tag integer NOT NULL
);


ALTER TABLE public.word_semantic OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 16902)
-- Name: word_semantic_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.word_semantic_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.word_semantic_id_seq OWNER TO postgres;

--
-- TOC entry 3403 (class 0 OID 0)
-- Dependencies: 220
-- Name: word_semantic_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.word_semantic_id_seq OWNED BY public.word_semantic.id;


--
-- TOC entry 3200 (class 2604 OID 16887)
-- Name: grammar_tag id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.grammar_tag ALTER COLUMN id SET DEFAULT nextval('public.grammar_tag_id_seq'::regclass);


--
-- TOC entry 3195 (class 2604 OID 16852)
-- Name: non_terminal_symbol id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.non_terminal_symbol ALTER COLUMN id SET DEFAULT nextval('public.non_terminal_symbol_id_seq'::regclass);


--
-- TOC entry 3196 (class 2604 OID 16861)
-- Name: production id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.production ALTER COLUMN id SET DEFAULT nextval('public.production_id_seq'::regclass);


--
-- TOC entry 3201 (class 2604 OID 16896)
-- Name: semantic_tag id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.semantic_tag ALTER COLUMN id SET DEFAULT nextval('public.semantic_tag_id_seq'::regclass);


--
-- TOC entry 3198 (class 2604 OID 16878)
-- Name: word id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word ALTER COLUMN id SET DEFAULT nextval('public.word_id_seq'::regclass);


--
-- TOC entry 3204 (class 2604 OID 16926)
-- Name: word_grammar id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_grammar ALTER COLUMN id SET DEFAULT nextval('public.word_grammar_id_seq'::regclass);


--
-- TOC entry 3203 (class 2604 OID 16906)
-- Name: word_semantic id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_semantic ALTER COLUMN id SET DEFAULT nextval('public.word_semantic_id_seq'::regclass);


--
-- TOC entry 3385 (class 0 OID 16884)
-- Dependencies: 217
-- Data for Name: grammar_tag; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.grammar_tag (id, name) FROM stdin;
1	Il
2	Lo
3	La
4	I
5	Gli
6	Le
7	_
8	Un
9	Un'
10	Uno
11	Una
\.


--
-- TOC entry 3379 (class 0 OID 16849)
-- Dependencies: 211
-- Data for Name: non_terminal_symbol; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.non_terminal_symbol (id, name) FROM stdin;
1	Start
\.


--
-- TOC entry 3381 (class 0 OID 16858)
-- Dependencies: 213
-- Data for Name: production; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.production (id, non_terminal_symbol, production) FROM stdin;
3	1	<0:N:F:N:F:Santo> <1:O(0):T:O(0):T:Malaparola>
\.


--
-- TOC entry 3387 (class 0 OID 16893)
-- Dependencies: 219
-- Data for Name: semantic_tag; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.semantic_tag (id, name, sticky) FROM stdin;
1	Malaparola	t
2	Santo	f
3	Divino	f
4	Animale	t
5	Malattia	t
\.


--
-- TOC entry 3383 (class 0 OID 16875)
-- Dependencies: 215
-- Data for Name: word; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.word (id, content, non_repeatable) FROM stdin;
1	Dio	t
2	Madonna	t
3	Cristo	t
4	San Giuseppe	t
5	Gesù	t
6	Maria	t
7	Papa	t
8	Cane	t
9	Porco	t
10	Cagna	t
11	Tubercoloso	t
12	Tubercolosa	t
13	Porca	t
14	Suino	t
15	Suina	t
16	Assassino	t
17	Assassina	t
18	Megattera	t
\.


--
-- TOC entry 3391 (class 0 OID 16923)
-- Dependencies: 223
-- Data for Name: word_grammar; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.word_grammar (id, word, grammar_tag) FROM stdin;
1	1	1
2	1	8
3	2	3
4	2	11
5	3	1
6	3	8
7	4	1
8	4	8
9	5	1
10	5	8
11	6	3
12	6	11
13	7	1
14	7	8
15	8	1
16	8	8
17	9	1
18	9	8
19	10	3
20	10	11
21	11	1
22	11	8
23	12	3
24	12	11
25	13	3
26	13	11
27	14	1
28	14	8
29	15	3
30	15	11
31	16	1
32	16	8
33	17	3
34	17	11
35	18	3
36	18	11
\.


--
-- TOC entry 3389 (class 0 OID 16903)
-- Dependencies: 221
-- Data for Name: word_semantic; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.word_semantic (id, word, semantic_tag) FROM stdin;
1	1	2
2	1	3
3	2	2
4	2	3
5	3	2
6	3	3
7	5	2
8	5	3
9	4	2
10	6	2
11	7	2
12	8	1
13	8	4
14	9	1
15	9	4
16	10	1
17	10	4
18	13	1
19	13	4
20	14	1
21	14	4
22	15	1
23	15	4
24	16	1
25	17	1
26	11	1
27	11	5
28	12	1
29	12	5
37	18	1
38	18	4
\.


--
-- TOC entry 3404 (class 0 OID 0)
-- Dependencies: 216
-- Name: grammar_tag_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.grammar_tag_id_seq', 11, true);


--
-- TOC entry 3405 (class 0 OID 0)
-- Dependencies: 210
-- Name: non_terminal_symbol_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.non_terminal_symbol_id_seq', 2, true);


--
-- TOC entry 3406 (class 0 OID 0)
-- Dependencies: 212
-- Name: production_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.production_id_seq', 7, true);


--
-- TOC entry 3407 (class 0 OID 0)
-- Dependencies: 218
-- Name: semantic_tag_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.semantic_tag_id_seq', 7, true);


--
-- TOC entry 3408 (class 0 OID 0)
-- Dependencies: 222
-- Name: word_grammar_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.word_grammar_id_seq', 36, true);


--
-- TOC entry 3409 (class 0 OID 0)
-- Dependencies: 214
-- Name: word_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.word_id_seq', 18, true);


--
-- TOC entry 3410 (class 0 OID 0)
-- Dependencies: 220
-- Name: word_semantic_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.word_semantic_id_seq', 38, true);


--
-- TOC entry 3217 (class 2606 OID 16891)
-- Name: grammar_tag grammar_tag_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.grammar_tag
    ADD CONSTRAINT grammar_tag_name_key UNIQUE (name);


--
-- TOC entry 3219 (class 2606 OID 16889)
-- Name: grammar_tag grammar_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.grammar_tag
    ADD CONSTRAINT grammar_tag_pkey PRIMARY KEY (id);


--
-- TOC entry 3206 (class 2606 OID 16856)
-- Name: non_terminal_symbol non_terminal_symbol_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.non_terminal_symbol
    ADD CONSTRAINT non_terminal_symbol_name_key UNIQUE (name);


--
-- TOC entry 3208 (class 2606 OID 16854)
-- Name: non_terminal_symbol non_terminal_symbol_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.non_terminal_symbol
    ADD CONSTRAINT non_terminal_symbol_pkey PRIMARY KEY (id);


--
-- TOC entry 3212 (class 2606 OID 16866)
-- Name: production production_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.production
    ADD CONSTRAINT production_pkey PRIMARY KEY (id);


--
-- TOC entry 3221 (class 2606 OID 16901)
-- Name: semantic_tag semantic_tag_name_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.semantic_tag
    ADD CONSTRAINT semantic_tag_name_key UNIQUE (name);


--
-- TOC entry 3223 (class 2606 OID 16899)
-- Name: semantic_tag semantic_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.semantic_tag
    ADD CONSTRAINT semantic_tag_pkey PRIMARY KEY (id);


--
-- TOC entry 3233 (class 2606 OID 16928)
-- Name: word_grammar word_grammar_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_grammar
    ADD CONSTRAINT word_grammar_pkey PRIMARY KEY (id);


--
-- TOC entry 3215 (class 2606 OID 16881)
-- Name: word word_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word
    ADD CONSTRAINT word_pkey PRIMARY KEY (id);


--
-- TOC entry 3228 (class 2606 OID 16908)
-- Name: word_semantic word_semantic_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_semantic
    ADD CONSTRAINT word_semantic_pkey PRIMARY KEY (id);


--
-- TOC entry 3209 (class 1259 OID 16872)
-- Name: idx_production_non_terminal_symbol; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_production_non_terminal_symbol ON public.production USING btree (non_terminal_symbol);


--
-- TOC entry 3210 (class 1259 OID 16873)
-- Name: idx_production_nts_amount; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_production_nts_amount ON public.production USING btree (nts_amount);


--
-- TOC entry 3213 (class 1259 OID 16882)
-- Name: idx_word_content; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_word_content ON public.word USING btree (content varchar_pattern_ops);


--
-- TOC entry 3229 (class 1259 OID 16940)
-- Name: idx_word_grammar_grammar_tag; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_word_grammar_grammar_tag ON public.word_grammar USING btree (grammar_tag);


--
-- TOC entry 3230 (class 1259 OID 16941)
-- Name: idx_word_grammar_uniqueness; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_word_grammar_uniqueness ON public.word_grammar USING btree (word, grammar_tag);


--
-- TOC entry 3231 (class 1259 OID 16939)
-- Name: idx_word_grammar_word; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_word_grammar_word ON public.word_grammar USING btree (word);


--
-- TOC entry 3224 (class 1259 OID 16920)
-- Name: idx_word_semantic_semantic_tag; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_word_semantic_semantic_tag ON public.word_semantic USING btree (semantic_tag);


--
-- TOC entry 3225 (class 1259 OID 16921)
-- Name: idx_word_semantic_uniqueness; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_word_semantic_uniqueness ON public.word_semantic USING btree (word, semantic_tag);


--
-- TOC entry 3226 (class 1259 OID 16919)
-- Name: idx_word_semantic_word; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_word_semantic_word ON public.word_semantic USING btree (word);


--
-- TOC entry 3234 (class 2606 OID 16867)
-- Name: production production_non_terminal_symbol_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.production
    ADD CONSTRAINT production_non_terminal_symbol_fkey FOREIGN KEY (non_terminal_symbol) REFERENCES public.non_terminal_symbol(id);


--
-- TOC entry 3238 (class 2606 OID 16934)
-- Name: word_grammar word_grammar_grammar_tag_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_grammar
    ADD CONSTRAINT word_grammar_grammar_tag_fkey FOREIGN KEY (grammar_tag) REFERENCES public.grammar_tag(id);


--
-- TOC entry 3237 (class 2606 OID 16929)
-- Name: word_grammar word_grammar_word_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_grammar
    ADD CONSTRAINT word_grammar_word_fkey FOREIGN KEY (word) REFERENCES public.word(id);


--
-- TOC entry 3236 (class 2606 OID 16914)
-- Name: word_semantic word_semantic_semantic_tag_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_semantic
    ADD CONSTRAINT word_semantic_semantic_tag_fkey FOREIGN KEY (semantic_tag) REFERENCES public.semantic_tag(id);


--
-- TOC entry 3235 (class 2606 OID 16909)
-- Name: word_semantic word_semantic_word_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.word_semantic
    ADD CONSTRAINT word_semantic_word_fkey FOREIGN KEY (word) REFERENCES public.word(id);


-- Completed on 2022-02-01 22:44:30 CET

--
-- PostgreSQL database dump complete
--

