/* Blocks of the status bar. dwmblocks writes them to the root window name,
 * which the penrose bar shows on the right. "pkill -RTMIN+<signal> dwmblocks"
 * updates a block right away. Recompile with make after editing. */
static const Block blocks[] = {
	/*Icon*/	/*Command*/		/*Update Interval*/	/*Update Signal*/
	{"",	"sb-network",	5,	4},
	{"",	"sb-recording",	0,	9},
	{"",	"sb-volume",	5,	10},
	{"",	"sb-battery",	5,	3},
	{"",	"sb-memory",	5,	14},
	{"",	"sb-clock",	1,	1},
};

/* Sets delimiter between status commands. NULL character ('\0') means no delimiter. */
static char *delim = " | ";
