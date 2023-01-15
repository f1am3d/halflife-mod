#pragma once

struct playermove_t_mod {
	public:
	float lastJumpTime = 0.0;
	float jumpDelay = 200.0;
	bool isRunning = false;
};