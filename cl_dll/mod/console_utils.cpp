#include "console_utils.h"
#include "../cl_util.h"


void ConsolePrintFloat( float& value ) {
	char buffer[64];
	snprintf( buffer, sizeof buffer, "%f\n", value );

	ConsolePrint( buffer );
}