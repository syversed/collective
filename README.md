# COLLECTIVE


## About

---
Collective 

## App Structure

---
Collective follows the below structure, for organization of its runtime files:

```
collective/
├── collective[.exe]
├── app/
│   ├── static/
│   │   ├── js
│   │   └── scss
│   ├── templates
│   ├── content/
│   │   ├── index.md
│   │   └── blog/
│   │       └── 2023-02-22-Example_Blogpost.md
│   └── live-site/
│       ├── index.html
│       └── blog/
│           └── 2023-02-22-Example_Blogpost.html
└── config/
    ├── log4rs.toml
    └── collective.toml
```

The following table describes the purpose of the above directories.

| Directory       | Purpose                                                                                                                                                                                                                                                                                                           |
|-----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `app`           | The application root.                                                                                                                                                                                                                                                                                             |
| `app/static`    | Static files served as part of the web server. Includes static CSS and JS.                                                                                                                                                                                                                                        |
| `app/templates` | Application templates for Collective. Written in Tera, all files should end with `.tera`.                                                                                                                                                                                                                         |
| `app/content`   | User content.<br>Files in this directory should be valid Markdown, and end in `.md`.<br>On startup, these files will be used to compose `app/live-site`.                                                                                                                                                          |
| `app/live-site` | The live, running copy of the current Collective site.<br>When Collective is not running, this directory will be empty. On startup, this directory<br>is regenerated and loaded, and on shutdown, it is cleared.<br><br>You should not edit files in this directory directly; changes may be reset by Collective. |
| `config`        | Application configuration files.                                                                                                                                                                                                                                                                                  |
