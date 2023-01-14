#include <corecrt_math.h>
#include <mathlib.h>

#include "view_utils.h"

float getStepFactor( float& globalTime, float& stepTime ) {
	float msTime = globalTime * 1000;
	float stepFactor = 0.0;
	float stepMod = fmod( msTime, stepTime );

	if( stepMod < (stepTime / 2.0) ) {
		stepFactor = NormalizeValue( stepMod, stepTime );
	}
	else {
		stepFactor = NormalizeValue( stepTime - stepMod, stepTime );
	}

	stepFactor = NormalizeValue( stepFactor, 0.5 ) - 0.5;

	return stepFactor;
}

float getHorizontalVelocityFactor( Vector2D& velocity ) {
    return abs(velocity.Length());
}

