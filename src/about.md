# **.\src**

## **== .\src\parser.rs ==**

#### *About* [constants](parser.rs#L6)
|COMMENT_PATTERN| - used to find TODO, NOTE and FIXME comments\
|ABOUT_PATTERN| - used to find 'About' line\
|DETAILS_PATTERN| - used to separate comment symbols (//) and text\
|OBJECT_PATTERN| - used to find object (variables, and func names) in line

---

#### *About* [parse_line()](parser.rs#L38)
used to parse 1 line\
if line is matched [COMMENT_PATTERN](parser.rs#L7), it called [parse_comment()](parser.rs#L71)\
if line is matched [ABOUT_PATTERN](parser.rs#L8), it called [parse_about()](parser.rs#L87)

---

#### *About* [|parse_comment()|](parser.rs#L71)
uset to find TODO, NOTE and FIXME comments\
it's eather creates a new comment, and insert it, or returns an old one

---

#### *About* [|parse_about()|](parser.rs#L87)
used to find 'about' blocks in the file to save them\
return storage::About

---

## **== .\src\storage.rs ==**

#### *About* [|Storage|](storage.rs#L8)
main data sctructure. Contains 2 maps: files, and tasks

---

#### *About* [|About|](storage.rs#L21)
Here is how 'About' blocks saved\
|header| - string with "About' in line\
|index| - number of header line in file\
|lines| - vector of lines

---

#### *About* [|Task|](storage.rs#L37)
Used to save local task\
|id| - used to work with taks (`chant done <id>` and `chant remove <id>`). It just a first 6 digits of hash\
|text| - just a content\
|done| - is it done or not

---

#### *About* [File](storage.rs#L54)
Created for each file, that scanner checks. It contains all of necessary data for chant to work with\
|hash| - used to chech if file changed or not\
|path| - path from root of repo to the file\
|dir| - relative path from directory to file\
|comments| - map of found comments (todo, note and fixme). It used hash as key, so I can change if line is changed or not\
|abouts| - vector of 'about' blocks

---

#### *About* [Comment](storage.rs#L81)
Contains a data about every comment\
|id| - now used now\
|kind| - TODO / NOTE / FIXME\
|index| - line with commend\
|hash| - hash of the line

---
