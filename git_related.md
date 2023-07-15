git global config is present in /home/user folder and global git ignore @ /home/user/.config/git

you can push to multiple locations simultaneously if you just want to mirror code, and exclude releases etc.
To do this customise modify config file in the .git folder of you local repo and add:

[remote "name_to_refer_by"]
pushurl = https://codeberg.org/user/reponame1.git
pushurl = https://codeberg.org/user/reponame2.git

"git push name_to_refer_by" will push code to both repos