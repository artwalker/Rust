#### Rust Basic Programming One

1. **Comments in Rust:**
   Comments, ignored by compilers, enhance source code readability.
   - Rust comments start with double backslashes (`//`).

   ```rust
   // This is a single-line comment.
   // This is another single-line comment.
   ```

   - Multi-line comments in Rust use `/* ... */` to span across multiple lines.

   ```rust
   /* This is a multi-line comment.
      This is another line of the comment. */
   ```

2. **Basic Print Commands:**
   - The `print!` command outputs text without a new line, while `println!` includes a new line after printing.

   ```rust
   print!("This is a print statement.");
   println!("This is a println statement.");
   ```

   ![](此处有插图 BasicPrintCommands.png)
   - Syntax involves using placeholders (`{}`) inside double quotes to replace them with provided values.

   ```rust
   println!("{} is learning {}.", "Ethan", "Rust");
   ```

   Printing strings and numbers requires correct placeholder formatting for their respective values.

3. **Escape Sequences:**
   - Escape sequences like `\n` (new line), `\t` (tab), and `\r` (carriage return) offer control over text layout.
   - `\\` prints a backslash, while `\'` and `\"` print single and double quotes inside a string.

4. **Positional and Named Arguments:**
   - Placeholders can be filled using positional arguments (`{}`, `{}`, `{}`) or named arguments (`{language}`, `{activity}`).

5. **Printing Basic Math Operations:**
   - Utilizing placeholders allows printing the result of mathematical operations within a print statement.

This overview touches on Rust's commenting conventions, basic print commands, escape sequences, argument handling, and printing mathematical results within Rust's print statements. Further details on data types will be covered in subsequent lectures.

---

#### Rust 基础编程一

这一节介绍了 Rust 的基础编程。适用于熟悉其他编程语言的人，提供了 Rust 编码的快速入门指南。即使对于初学者，该部分也从零开始详细讲解每个步骤。

1. **Rust 中的注释：**
   - 注释在编译器中被忽略，有助于提高源代码的可读性。
   - Rust 中的注释以双斜杠（`//`）开头。

   多行注释使用 `/* ... */` 跨越多行。

2. **基本的打印命令：**
   - `print!` 命令输出文本但不换行，而 `println!` 在打印后换行。
   - 语法涉及在双引号内使用占位符（`{}`），以替换为提供的值。

   打印字符串和数字需要正确的占位符格式。

3. **转义序列：**
   - 转义序列如 `\n`（新行）、`\t`（制表符）和 `\r`（回车）控制文本布局。
   - `\\` 打印反斜杠，`\'` 和 `\"` 在字符串内打印单引号和双引号。

4. **位置参数和命名参数：**
   - 占位符可以使用位置参数（`{}`、`{}`、`{}`）或命名参数（`{language}`、`{activity}`）填充。

5. **打印基本数学运算：**
   - 利用占位符允许在打印语句中打印数学运算的结果。

这份概述涉及 Rust 的注释约定、基本打印命令、转义序列、参数处理以及在 Rust 的打印语句中打印数学结果。后续的课程将涵盖更多关于数据类型的细节。
