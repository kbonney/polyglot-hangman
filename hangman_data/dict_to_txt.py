def convert_hunspell_to_text(hunspell_file, output_file):
    with open(hunspell_file, 'r', encoding='utf-8') as infile, open(output_file, 'w', encoding='utf-8') as outfile:
        for line in infile:
            # Extract the word part before the '/'
            word = line.split('/')[0].strip()
            if word and not any(i.isdigit() for i in word) and len(word) > 2:
                outfile.write(word.lower() + '\n')


if __name__=="__main__":
    hunspell_file = "en_US-custom.dic"
    output_file = "words.txt"
    convert_hunspell_to_text(hunspell_file, output_file)