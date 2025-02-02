alter table revisions
    add constraint revisions_page_id_fkey
        foreign key (page_id)
        references pages (id);
